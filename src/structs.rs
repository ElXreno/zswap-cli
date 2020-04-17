use crate::utils;
use clap::ArgMatches;
use serde_derive::Deserialize;

pub struct ZswapDebugParam {
    pub name: String,
    pub sys_value: Option<i32>,
}

pub struct ZswapDebugParams {
    pub params: Vec<ZswapDebugParam>,
}

pub struct ZswapParam {
    pub name: String,
    pub value: Option<String>,
    pub sys_value: Option<String>,
}

pub struct ZswapParams {
    pub params: Vec<ZswapParam>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ZswapParamsConf {
    #[serde(rename = "ZSWAP_ACCEPT_THRESHOLD_PERCENT")]
    pub accept_threshold_percent: String,
    #[serde(rename = "ZSWAP_COMPRESSOR")]
    pub compressor: String,
    #[serde(rename = "ZSWAP_ENABLED")]
    pub enabled: String,
    #[serde(rename = "ZSWAP_MAX_POOL_PERCENT")]
    pub max_pool_percent: String,
    #[serde(rename = "ZSWAP_SAME_FILLED_PAGES_ENABLED")]
    pub same_filled_pages_enabled: String,
    #[serde(rename = "ZSWAP_ZPOOL")]
    pub zpool: String,
}

impl Default for ZswapParamsConf {
    fn default() -> Self {
        ZswapParamsConf {
            accept_threshold_percent: "".to_string(),
            compressor: "".to_string(),
            enabled: "".to_string(),
            max_pool_percent: "".to_string(),
            same_filled_pages_enabled: "".to_string(),
            zpool: "".to_string(),
        }
    }
}

impl Default for ZswapDebugParam {
    fn default() -> Self {
        ZswapDebugParam {
            name: "".to_string(),
            sys_value: None,
        }
    }
}

impl ZswapDebugParams {
    pub fn load_sys_params() -> Self {
        utils::read_debug_params()
    }
}

impl ZswapParams {
    pub fn load_sys_params() -> Self {
        utils::read_sys_params()
    }
    pub fn load_params_from_config(&mut self) -> &mut Self {
        let zswap_params_conf = utils::read_config();
        if zswap_params_conf.is_some() {
            let zswap_params_conf = zswap_params_conf.unwrap();

            // TODO: Need fix this trash code
            self.set_param(
                "accept_threshold_percent",
                Some(zswap_params_conf.accept_threshold_percent),
            );
            self.set_param("compressor", Some(zswap_params_conf.compressor));
            self.set_param("enabled", Some(zswap_params_conf.enabled));
            self.set_param("max_pool_percent", Some(zswap_params_conf.max_pool_percent));
            self.set_param(
                "same_filled_pages_enabled",
                Some(zswap_params_conf.same_filled_pages_enabled),
            );
            self.set_param("zpool", Some(zswap_params_conf.zpool));
        }

        self
    }
    pub fn load_params_from_matches(&mut self, matches: &ArgMatches) -> &mut Self {
        for param in utils::constants::PARAM_LIST.iter() {
            if matches.is_present(param) {
                self.set_param(param, Some(matches.value_of(param).unwrap().to_string()));
            }
        }

        self
    }
    pub fn save(&self) {
        utils::save_sys_params(self);
    }
    fn set_param(&mut self, param: &str, value: Option<String>) -> &mut Self {
        let position = self.params.iter().position(|x| x.name == param.to_string());
        if position.is_some() {
            let position = position.unwrap();
            self.params[position].value = value;
        } else {
            warn!("Can't find {} in sys_params", param);
        }

        self
    }
}
