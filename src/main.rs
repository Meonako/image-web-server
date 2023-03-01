mod config;
mod path;
mod structs;
mod utils;

use std::sync::RwLock;

use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpServer};

use colored::Colorize;
use structs::AppState;

const INDEX_DEFAULT: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index | Images Server</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            background-color: black;
            height: 100vh;
        }

        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100%;
        }

        button {
            font-size: large;
            background-color: inherit;
            border: 1px solid white;
            color: white;
            padding: 1rem 2rem;
            transition: .5s;
            margin-top: 1rem;
            width: 50%;
        }

        button:hover {
            background-color: rgb(119, 119, 119);
            color: black;
        }

        @media (max-width: 500px) {
            button {
                width: 100%;
            }
        }
    </style>
</head>
<body>
    {{ data }}

    <script>
        function handle(btn) {
            window.location.href = `/${btn.id}/1`
        }
    </script>
</body>
</html>
"##;

const HTML_DEFAULT: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Images</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            background-color: black;
        }

        img {
            max-width: 100%;
            max-height: 100%;
            margin: 0 auto;
        }

        .nav {
            position: sticky;
            top: 0;
            z-index: 1;
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;
        }

        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
    </style>
</head>
<body>
    <div class="nav">
        <a href="{{ next page }}">Next Page</a>
        <a href="{{ previous page }}">Previous Page</a>
        <a href="{{ last page }}">Last Page</a>
        <button id="reload-btn">Reload</button>
    </div>

    {{ data }}

    <script>
        function main() {
            const reloadBtn = document.querySelector("#reload-btn");
            if (!reloadBtn) return;

            reloadBtn.addEventListener('click', async () => {
                const response = await fetch('{{ reload path }}');
                if (!response.ok) return;

                location.reload();
            })
        }

        main()
    </script>
</body>
</html>
"##;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    colored::control::set_virtual_terminal(true).unwrap();

    let _logger = flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .format_for_files(utils::file_logger)
        .format_for_stdout(utils::stdout_logger)
        .log_to_file(flexi_logger::FileSpec::default().directory("iws.log"))
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .print_message()
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .start()
        .unwrap();

    let config = config::init();
    let path_list = config.get_enable_path();

    if path_list.is_empty() {
        log::error!("All path is disable.");
        panic!("All path is disable");
    }

    log::info!("Binding on: {}", config.address);
    log::info!("Images per page: {}", config.images_per_page);

    // Create state here to achieve globally shared state
    // it must be created outside of the closure passed to HttpServer::new and moved/cloned in.
    // If not, the state might be desync
    let app_data = web::Data::new(AppState {
        txt2img: 
            if config.txt2img.enable {
                Some(RwLock::new(utils::read_directory(&config.txt2img.path)))
            } else {
                None
            },
        txt2img_grid: 
            if config.txt2img_grid.enable {
                Some(RwLock::new(utils::read_directory(
                    &config.txt2img_grid.path,
                )))
            } else {
                None
            },
        img2img: 
            if config.img2img.enable {
                Some(RwLock::new(utils::read_directory(&config.img2img.path)))
            } else {
                None
            },
        img2img_grid: 
            if config.img2img_grid.enable {
                Some(RwLock::new(utils::read_directory(
                    &config.img2img_grid.path,
                )))
            } else {
                None
            },
        extras: 
            if config.extras.enable {
                Some(RwLock::new(utils::read_directory(&config.extras.path)))
            } else {
                None
            },
        config: config.clone(),
    });

    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(app_data.clone())
            .wrap(
                actix_web::middleware::Logger::new(
                    "[ %{METHOD}xi %{STATUS}xo ] %{PATH}xi - %T seconds",
                )
                .custom_request_replace("METHOD", |req| {
                    req.method().to_string().bright_cyan().to_string()
                })
                .custom_response_replace("STATUS", |res| match res.status() {
                    StatusCode::OK => "200".green().to_string(),
                    StatusCode::NOT_MODIFIED => "304".blue().to_string(),
                    StatusCode::NOT_FOUND => "404".red().to_string(),
                    x => x.to_string(),
                })
                .custom_request_replace("PATH", |req| match req.path() {
                    x if x.len() > 20 => x[..=20].to_owned() + "...",
                    x => x.to_owned(),
                }),
            )
            .service(path::index);

        let mut reload_scope = web::scope("/reload");
        let mut assets_scope = web::scope("/assets");

        if config.txt2img.enable {
            app = app.service(path::txt_to_img);
            reload_scope = reload_scope.service(path::reload::reload_txt2img);
        }

        if config.txt2img_grid.enable {
            app = app.service(path::txt_to_img_grids);
            reload_scope = reload_scope.service(path::reload::reload_txt2img_grid);
        }

        if config.img2img.enable {
            app = app.service(path::img_to_img);
            reload_scope = reload_scope.service(path::reload::reload_img2img);
        }

        if config.img2img_grid.enable {
            app = app.service(path::img_to_img_grids);
            reload_scope = reload_scope.service(path::reload::reload_img2img_grid);
        }

        if config.extras.enable {
            app = app.service(path::extras_images);
            reload_scope = reload_scope.service(path::reload::reload_extras);
        }

        for path in path_list.iter() {
            let spliter = 
                if path.contains("/") {
                    "/"
                } else if path.contains("\\") {
                    "\\"
                } else {
                    panic!("Path does not contains either / or \\.")
                };

            let folder_name = path.split(spliter).last().unwrap();

            assets_scope = assets_scope.service(Files::new(&format!("/{}", folder_name), path));
        }

        app.service(reload_scope).service(assets_scope)
    })
    .bind(config.address)?
    .workers(20)
    .run()
    .await
}
