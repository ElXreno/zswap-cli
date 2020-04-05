use crate::utils;
use clap::ArgMatches;

pub struct ZswapParam {
    pub name: String,
    pub value: Option<String>,
    pub sys_value: Option<String>,
}

pub struct ZswapParams {
    pub params: Vec<ZswapParam>,
}

impl ZswapParams {
    pub fn load_sys_params() -> Self {
        utils::read_sys_params()
    }
    pub fn load_params_from_matches(&mut self, matches: &ArgMatches) -> &mut Self {
        let params = vec![
            "accept_threshold_percent",
            "compressor",
            "enabled",
            "max_pool_percent",
            "same_filled_pages_enabled",
            "zpool",
        ];

        for param in params {
            let position = self.params.iter().position(|x| x.name == param.to_string());
            if position.is_some() {
                if matches.is_present(param) {
                    let position = position.unwrap();
                    self.params[position].value =
                        Some(matches.value_of(param).unwrap().to_string());
                } else {
                    info!("Param {} is not given", param);
                }
            } else {
                warn!("Can't find {} in sys_params", param);
            }
        }

        self
    }
    pub fn save(&self) {
        utils::save_sys_params(self);
    }
}
