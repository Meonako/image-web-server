use actix_web::{get, web, Responder};

use crate::path::page_common;
use crate::structs::AppState;

#[get("/txt2img/{page}")]
pub async fn txt_to_img(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let images_list = { data.txt2img.as_ref().unwrap().read().unwrap() };
    page_common(&data, &images_list, path.into_inner(), "txt2img-images", "/reload/txt2img")
}