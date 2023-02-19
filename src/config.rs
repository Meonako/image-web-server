use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    io::ErrorKind,
};

const CONFIG_FILE: &str = "iws.config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub images_folder: String,
    pub images_per_page: usize,
    pub html_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            images_folder: "./outputs/txt2img-images".to_owned(),
            images_per_page: 100,
            html_file: "format.html".to_owned(),
        }
    }
}

pub fn init() -> Config {
    let data = match read_to_string(CONFIG_FILE) {
        Ok(str) => str,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let default_value = match serde_json::to_vec_pretty(&Config::default()) {
                    Ok(json) => json,
                    Err(e) => {
                        log::error!("Cannot convert default value to json: {:?}", e);
                        return Config::default();
                    }
                };

                match write(CONFIG_FILE, default_value) {
                    Ok(_) => log::info!("Config file created ({CONFIG_FILE})"),
                    Err(_) => log::error!("Cannot create and write to config file"),
                };
            }

            return Config::default();
        }
    };

    match serde_json::from_str::<Config>(&data) {
        Ok(val) => val,
        Err(_) => Config::default(),
    }
}
