extern crate libc;

use libc::{sysconf, _SC_PAGESIZE};
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

pub mod constants;

use crate::structs::{ZswapDebugParam, ZswapDebugParams, ZswapParam, ZswapParams, ZswapParamsConf};
use std::process::exit;

pub fn read_config() -> Option<ZswapParamsConf> {
    let config_str = fs::read_to_string(constants::CONFIG_PATH).unwrap_or("".to_string());

    match serde_ini::from_str::<ZswapParamsConf>(&config_str) {
        Ok(s) => Some(s),
        Err(e) => {
            warn!(
                "Failed to read config file! Path: {}, error: {}",
                constants::CONFIG_PATH,
                e
            );
            None
        }
    }
}

// TODO: Optimize this very ugly and trashy code

pub fn read_sys_params() -> ZswapParams {
    let mut params = ZswapParams { params: vec![] };
    let files = get_files(constants::ZSWAP_BASEPATH);

    for file in files {
        params.params.push(read_sys_param(file));
    }

    params
}

pub fn read_sys_param(param_name: String) -> ZswapParam {
    let path = format!("{}/{}", constants::ZSWAP_BASEPATH, param_name);
    let sys_value = fs::read_to_string(&path)
        .expect(format!("Can't read {} file", path).as_str())
        .trim()
        .to_string();

    let param = ZswapParam {
        name: param_name,
        value: None,
        sys_value: Some(sys_value),
    };

    param
}

pub fn read_debug_params() -> ZswapDebugParams {
    let mut params = ZswapDebugParams { params: vec![] };
    let files = get_files(constants::ZSWAP_DEBUG_BASEPATH);

    for file in files {
        params.params.push(read_debug_param(file));
    }

    params
}

pub fn read_debug_param(param_name: String) -> ZswapDebugParam {
    let path = format!("{}/{}", constants::ZSWAP_DEBUG_BASEPATH, param_name);
    let sys_value = fs::read_to_string(&path)
        .expect(format!("Can't read {} file", path).as_str())
        .trim()
        .to_string();

    let param = ZswapDebugParam {
        name: param_name,
        sys_value: sys_value.parse::<i32>().ok(),
    };

    param
}

pub fn save_sys_params(params: &ZswapParams) {
    // TODO: Do not show this when nothing to save
    info!("Saving params...");
    for param in &params.params {
        if param.value.is_some()
            && param.value.as_ref().unwrap() != ""
            && param.sys_value.is_some()
            && param
                .sys_value
                .as_ref()
                .expect(format!("Can't unwrap sys value for param {}", param.name).as_str())
                != param.value.as_ref().unwrap()
        {
            match save_sys_param(&param) {
                Ok(_) => info!(
                    "Successfully saved param {} with value {}. Old value is {}",
                    param.name,
                    param.value.as_ref().unwrap(),
                    param.sys_value.as_ref().unwrap()
                ),
                Err(_) => error!(
                    "Failed to save param {} with value {}! System value is {}",
                    param.name,
                    param.value.as_ref().unwrap(),
                    param.sys_value.as_ref().unwrap()
                ),
            }
        } else {
            info!("Ignoring param {}", param.name);
        }
    }
    info!("Done!");
}

pub fn save_sys_param(param: &ZswapParam) -> Result<(), Error> {
    let value = param.value.as_ref().unwrap();

    File::create(format!("{}/{}", constants::ZSWAP_BASEPATH, param.name))
        .unwrap()
        .write_all(value.as_bytes())
}

pub fn is_root() -> bool {
    // FIXME: Should I check read/write permission instead of user matching?
    whoami::user() == String::from("root")
}

pub fn check_root() {
    if !is_root() {
        error!("You are not a root user!");
        exit(1);
    }
}

fn get_files(dir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(dir).expect(format!("Can't read {} dir!", dir).as_str()) {
        let entry = entry.expect("Can't unwrap entry");
        if entry.path().is_file() {
            files.push(
                entry.file_name().into_string().expect(
                    format!(
                        "Can't convert OsString to String! OsString: {:?}",
                        entry.file_name()
                    )
                    .as_str(),
                ),
            )
        }
    }

    files
}

pub fn get_page_size() -> usize {
    unsafe { sysconf(_SC_PAGESIZE) as usize }
}
