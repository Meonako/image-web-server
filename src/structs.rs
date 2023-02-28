use std::sync::RwLock;

use crate::config;

pub struct AppState {
    pub txt2img:        Option<RwLock<Vec<String>>>,
    pub txt2img_grid:   Option<RwLock<Vec<String>>>,
    pub img2img:        Option<RwLock<Vec<String>>>,
    pub img2img_grid:   Option<RwLock<Vec<String>>>,
    pub extras:         Option<RwLock<Vec<String>>>,
    pub config:         config::Config,
}