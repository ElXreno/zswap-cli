use std::fs;
use std::fs::File;
use std::io::{Error, Write};

pub mod constants;

pub fn read_param(param: &str) -> String {
    fs::read_to_string(format!("{}/{}", constants::ZSWAP_BASEPATH, param))
        .unwrap_or("NaN".parse().unwrap())
        .trim()
        .to_string()
}

pub fn save_param(param: &str, value: String) -> Result<(), Error> {
    println!(
        "file: {} | value: {}",
        format!("{}/{}", constants::ZSWAP_BASEPATH, param),
        value
    );
    File::create(format!("{}/{}", constants::ZSWAP_BASEPATH, param))
        .unwrap()
        .write_all(value.as_bytes())
}
