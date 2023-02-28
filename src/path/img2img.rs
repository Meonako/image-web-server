use actix_web::{get, web, Responder};

use crate::path::page_common;
use crate::structs::AppState;

#[get("/img2img/{page}")]
pub async fn img_to_img(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let images_list = { data.img2img.as_ref().unwrap().read().unwrap() };
    page_common(&data, &images_list, path.into_inner(), "img2img-images", "/reload/img2img")
}