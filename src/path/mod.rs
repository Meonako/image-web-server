mod txt2img;
mod txt2img_grid;
mod img2img;
mod img2img_grid;
mod extras;

pub mod reload;

pub use txt2img::txt_to_img;
pub use txt2img_grid::txt_to_img_grids;
pub use img2img::img_to_img;
pub use img2img_grid::img_to_img_grids;
pub use extras::extras_images;

use actix_web::{get, Responder, web, HttpResponse};

use crate::{structs::AppState, utils};

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let index_file_path = &data.config.index_file;
    let html = utils::get_basic_html(index_file_path, crate::INDEX_DEFAULT);

    let mut html_data = String::from(r#"<div class="container">"#);

    if data.config.txt2img.enable {
        html_data.push_str(r#"<button id="txt2img" onclick="handle(this)">Txt2Img</button>"#);
    }

    if data.config.txt2img_grid.enable {
        html_data.push_str(r#"<button id="txt2img-grids" onclick="handle(this)">Txt2Img Grids</button>"#);
    }

    if data.config.img2img.enable {
        html_data.push_str(r#"<button id="img2img" onclick="handle(this)">Img2Img</button>"#);
    }

    if data.config.img2img_grid.enable {
        html_data.push_str(r#"<button id="img2img-grids" onclick="handle(this)">Img2Img Grids</button>"#);
    }

    if data.config.extras.enable {
        html_data.push_str(r#"<button id="extras" onclick="handle(this)">Extras</button>"#);
    }

    html_data.push_str("</div>");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.replace("{{ data }}", &html_data))
}

fn page_common(data: &web::Data<AppState>, images_list: &Vec<String>, page: usize, path_prefix: &str, reload_path: &str) -> impl Responder {
    let html_file_path = &data.config.html_file;
    let html = utils::get_basic_html(html_file_path, crate::HTML_DEFAULT);

    let limit_per_page = data.config.images_per_page;

    let total_files = images_list.len();

    let remainder = total_files % limit_per_page;
    let last_page = match remainder {
        0 => total_files / limit_per_page,
        _ => total_files / limit_per_page + 1,
    };

    if page > last_page {
        log::warn!("Navigate page: {} > Last Page: {}", page, last_page);
        return HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not Found!");
    }

    let mut data = String::from(r#"<div class="container">"#);

    let last_index = match page {
        pg if pg == last_page => (pg - 1) * limit_per_page + remainder,
        pg => pg * limit_per_page,
    };

    let start = last_index - (limit_per_page - 1);

    for image in &images_list[start..last_index] {
        let path: Vec<&str> = image.split('\\').collect();
        let actual_files = path.last().unwrap();
        data += format!(r#"<img src="/assets/{}/{}"/>"#, path_prefix, actual_files).as_str()
    }

    data.push_str("</div>");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            html
                .replace(
                    "{{ next page }}",
                    &format!("{}", if page < last_page {
                        page + 1
                    } else {
                        last_page
                    }),
                )
                .replace(
                    "{{ previous page }}", 
                    &format!("{}", if page <= 1 {
                        1
                    } else {
                        page - 1
                    })
                )
                .replace("{{ last page }}", last_page.to_string().as_str())
                .replace("{{ data }}", &data)
                .replace("{{ reload path }}", reload_path)
        )
}