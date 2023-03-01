use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    io::ErrorKind,
};

const CONFIG_FILE: &str = "iws.config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub address:            String,
    pub txt2img:            Txt2Img,
    pub txt2img_grid:       Txt2ImgGrid,
    pub img2img:            Img2Img,
    pub img2img_grid:       Img2ImgGrid,
    pub extras:             Extras,
    pub images_per_page:    usize,
    pub index_file:         String,
    pub html_file:          String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Txt2Img {
    pub enable: bool,
    pub path:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Txt2ImgGrid {
    pub enable: bool,
    pub path:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Img2Img {
    pub enable: bool,
    pub path:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Img2ImgGrid {
    pub enable: bool,
    pub path:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extras {
    pub enable: bool,
    pub path:   String,
}

impl Config {
    pub fn get_enable_path(&self) -> Vec<String> {
        let mut path_list = Vec::new();

        if self.txt2img.enable {
            path_list.push(self.txt2img.path.clone())
        }

        if self.txt2img_grid.enable {
            path_list.push(self.txt2img_grid.path.clone())
        }

        if self.img2img.enable {
            path_list.push(self.img2img.path.clone())
        }

        if self.img2img_grid.enable {
            path_list.push(self.img2img_grid.path.clone())
        }

        if self.extras.enable {
            path_list.push(self.extras.path.clone())
        }

        path_list
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: "0.0.0.0:80".to_owned(),
            txt2img: Txt2Img {
                enable: true,
                path: "./outputs/txt2img-images".to_owned(),
            },
            txt2img_grid: Txt2ImgGrid {
                enable: false,
                path: "./outputs/txt2img-grids".to_owned(),
            },
            img2img: Img2Img {
                enable: true,
                path: "./outputs/img2img-images".to_owned(),
            },
            img2img_grid: Img2ImgGrid {
                enable: false,
                path: "./outputs/img2img-grids".to_owned(),
            },
            extras: Extras {
                enable: true,
                path: "./outputs/extras-images".to_owned(),
            },
            images_per_page: 100,
            index_file: "index.html".to_owned(),
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

    match serde_json::from_str(&data) {
        Ok(val) => val,
        Err(_) => Config::default(),
    }
}