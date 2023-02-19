use std::sync::RwLock;

use crate::config;

pub struct AppState {
    pub sync_folder: RwLock<Vec<String>>,
    pub config: config::Config,
}