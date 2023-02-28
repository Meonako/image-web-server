use actix_web::HttpResponse;
use actix_web::{get, web, Responder};

use crate::structs::AppState;
use crate::utils;

#[get("/img2img-grids")]
pub async fn reload_img2img_grid(data: web::Data<AppState>) -> impl Responder {
    if !data.config.img2img_grid.enable {
        return HttpResponse::NotFound();
    }

    let image_path_list = utils::read_directory(&data.config.img2img_grid.path);

    {
        let mut old_images_list = data.img2img_grid.as_ref().unwrap().write().unwrap();
        log::info!(
            "Old Data: {} | New Data: {}",
            old_images_list.len(),
            image_path_list.len()
        );
        *old_images_list = image_path_list;
    }

    HttpResponse::Ok()
}