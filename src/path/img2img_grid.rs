use actix_web::{get, web, Responder};

use crate::path::page_common;
use crate::structs::AppState;

#[get("/img2img-grids/{page}")]
pub async fn img_to_img_grids(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let images_list = { data.img2img_grid.as_ref().unwrap().read().unwrap() };
    page_common(&data, &images_list, path.into_inner(), "img2img-grids", "/reload/img2img-grids")
}