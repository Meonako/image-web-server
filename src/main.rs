mod config;
mod structs;
mod utils;

use std::sync::RwLock;

use actix_files::Files;
use actix_web::web::Redirect;
use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};

use structs::AppState;

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
    {{ data }}

    <script>
        function main() {
            const reloadBtn = document.querySelector("#reload-btn");
            if (!reloadBtn) return;

            reloadBtn.addEventListener('click', async () => {
                const response = await fetch('/reload');
                if (!response.ok) return;

                location.reload();
            })
        }

        main()
    </script>
</body>
</html>
"##;

#[get("/")]
async fn index() -> impl Responder {
    Redirect::to("/txt2img/1").permanent()
}

#[get("/txt2img/{page}")]
async fn txt_to_img(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let html_file_path = &data.config.html_file;
    let html = utils::get_basic_html(html_file_path);

    let folder_data = { data.sync_folder.read().unwrap() };
    let limit_per_page = data.config.images_per_page;

    let total_files = folder_data.len();

    let page = path.into_inner();

    let remainder = total_files % limit_per_page;
    let last_page = match remainder {
        0 => total_files / limit_per_page,
        _ => total_files / limit_per_page + 1,
    };

    if page > last_page {
        log::warn!("Navigate page: {} > lastpage: {}", page, last_page);
        return HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not Found!");
    }

    let mut data = format!(
        r#"
        <div class="nav">
            <a href="/txt2img/{}">Next Page</a>
            <a href="/txt2img/{}">Previous Page</a>
            <a href="/txt2img/{}">Last Page</a>
            <button id="reload-btn">Reload</button>
        </div>
        <div class="container">
        "#,
        if page < last_page {
            page + 1
        } else {
            last_page
        },
        page - 1,
        last_page,
    );

    let last_index = match page {
        pg if pg == last_page => (pg - 1) * limit_per_page + remainder,
        pg => pg * limit_per_page,
    };

    let start = last_index - (limit_per_page - 1);
    let end = last_index;

    for folder in &folder_data[start..end] {
        let path: Vec<&str> = folder.split('\\').collect();
        let actual_files = path.last().unwrap();
        data += format!(r#"<img src="/files/{}"/>"#, actual_files).as_str()
    }

    data.push_str("</div>");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.replace("{{ data }}", &data))
}

#[get("/reload")]
async fn reload(data: web::Data<AppState>) -> impl Responder {
    let path = data.config.images_folder.as_ref();

    let outputs_path = utils::read_directory(path);

    {
        let mut folder_data = data.sync_folder.write().unwrap();
        log::info!(
            "Old Data: {} | New Data: {}",
            folder_data.len(),
            outputs_path.len()
        );
        *folder_data = outputs_path;
    }

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    colored::control::set_virtual_terminal(true).unwrap();

    let _logger = 
        flexi_logger::Logger::try_with_str("info")
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
    let folder = config.images_folder.clone();

    log::info!("Images per page: {}", config.images_per_page);

    let outputs_path = utils::read_directory(&folder);

    // Create state here to achieve globally shared state
    // it must be created outside of the closure passed to HttpServer::new and moved/cloned in.
    // If not, the state might be desync
    let app_data = web::Data::new(AppState {
        sync_folder: RwLock::new(outputs_path),
        config,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(actix_web::middleware::Logger::new(
                "%s [ %r ] %a - %T seconds",
            ))
            .service(index)
            .service(txt_to_img)
            .service(reload)
            .service(Files::new("/files", folder.clone()))
    })
    .bind(("0.0.0.0", 80))?
    .workers(100)
    .run()
    .await
}
