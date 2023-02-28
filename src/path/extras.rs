use actix_web::{get, web, Responder};

use crate::path::page_common;
use crate::structs::AppState;

#[get("/extras/{page}")]
pub async fn extras_images(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let images_list = { data.extras.as_ref().unwrap().read().unwrap() };
    page_common(&data, &images_list, path.into_inner(), "extras-images", "/reload/extras")
}