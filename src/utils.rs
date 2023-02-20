use colored::Colorize;
use std::{
    fs::{read_dir, read_to_string},
    io::{Error, Write},
    path::Path,
};

pub fn read_directory(target_folder: &str) -> Vec<String> {
    log::info!("Reading folder: {}", target_folder);
    let outputs_folder = Path::new(target_folder);
    if !outputs_folder.exists() || !outputs_folder.is_dir() {
        log::error!("Outputs folder not found.");
        panic!("Outputs folder not found.");
    }

    let mut result = read_dir(outputs_folder)
        .expect("read outputs directory")
        .map(|r| {
            r.unwrap()
                .file_name()
                .to_str()
                .unwrap_or_default()
                .to_string()
        })
        .collect::<Vec<String>>();

    result.sort_by(|a, b| {
        let filename_split_a: Vec<&str> = a.split('-').collect();
        let filename_split_b: Vec<&str> = b.split('-').collect();

        let id_a = filename_split_a.first().unwrap().parse::<i32>().unwrap();
        let id_b = filename_split_b.first().unwrap().parse::<i32>().unwrap();

        id_a.cmp(&id_b)
    });

    result
}

pub fn get_basic_html(path: &str) -> String {
    match read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            println!("{}\nReturning Default HTML;", e);
            crate::HTML_DEFAULT.to_owned()
        }
    }
}

pub fn stdout_logger(
    w: &mut dyn Write,
    now: &mut flexi_logger::DeferredNow,
    record: &flexi_logger::Record<'_>,
) -> Result<(), Error> {
    write!(
        w,
        "{} [ {} ]: {}",
        now.format("%d/%m/%Y %H:%M:%S").to_string().bold().green(),
        match record.level() {
            lvl if lvl == flexi_logger::Level::Info => lvl.as_str().cyan(),
            lvl if lvl == flexi_logger::Level::Error => lvl.as_str().red(),
            lvl if lvl == flexi_logger::Level::Warn => lvl.as_str().yellow(),
            x => x.as_str().white(),
        },
        &record.args()
    )
}

pub fn file_logger(
    w: &mut dyn Write,
    now: &mut flexi_logger::DeferredNow,
    record: &flexi_logger::Record<'_>,
) -> Result<(), Error> {
    write!(
        w,
        "{} [ {} ]: {}",
        now.format("%d/%m/%Y %H:%M:%S").to_string(),
        record.level(),
        &record.args()
    )
}
