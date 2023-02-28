use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Ordering,
    fs::{read_dir, read_to_string},
    io::{Error, Write},
    path::Path,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

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
        if let Some(a_number) = RE.find(a) {
            if let Some(b_number) = RE.find(b) {
                let a_number = a_number.as_str().parse::<i32>().unwrap();
                let b_number = b_number.as_str().parse::<i32>().unwrap();

                a_number.cmp(&b_number)
            } else {
                log::error!("Not found number in b");
                Ordering::Equal
            }
        } else {
            log::error!("Not found number in a");
            Ordering::Equal
        }
    });

    result
}

pub fn get_basic_html(path: &str, def: &str) -> String {
    match read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            println!("{}\nReturning Default HTML;", e);
            def.to_owned()
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
