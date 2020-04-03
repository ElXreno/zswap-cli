use crate::utils;
use clap::ArgMatches;

pub struct ZswapConfig {
    pub accept_threshold_percent: String,
    pub compressor: String,
    pub enabled: String,
    pub max_pool_percent: String,
    pub same_filled_pages_enabled: String,
    pub zpool: String,
}

impl ZswapConfig {
    pub fn load_current_params() -> Self {
        ZswapConfig {
            accept_threshold_percent: utils::read_param("accept_threshold_percent").to_string(),
            compressor: utils::read_param("compressor").to_string(),
            enabled: utils::read_param("enabled").to_string(),
            max_pool_percent: utils::read_param("max_pool_percent").to_string(),
            same_filled_pages_enabled: utils::read_param("same_filled_pages_enabled").to_string(),
            zpool: utils::read_param("zpool").to_string(),
        }
    }
    pub fn load_from_matches(matches: &ArgMatches) -> Self {
        ZswapConfig {
            accept_threshold_percent: matches
                .value_of("accept_threshold_percent")
                .unwrap_or(utils::read_param("accept_threshold_percent").as_str())
                .to_string(),
            compressor: matches
                .value_of("compressor")
                .unwrap_or(utils::read_param("compressor").as_str())
                .to_string(),
            enabled: matches
                .value_of("enabled")
                .unwrap_or(utils::read_param("enabled").as_str())
                .to_string(),
            max_pool_percent: matches
                .value_of("max_pool_percent")
                .unwrap_or(utils::read_param("max_pool_percent").as_str())
                .to_string(),
            same_filled_pages_enabled: matches
                .value_of("same_filled_pages_enabled")
                .unwrap_or(utils::read_param("same_filled_pages_enabled").as_str())
                .to_string(),
            zpool: matches
                .value_of("zpool")
                .unwrap_or(utils::read_param("zpool").as_str())
                .to_string(),
        }
    }
    pub fn save(self) {
        utils::save_param("accept_threshold_percent", self.accept_threshold_percent)
            .expect("Couldn't save accept_threshold_percent");

        utils::save_param("compressor", self.compressor).expect("Couldn't save compressor");

        utils::save_param("enabled", self.enabled).expect("Couldn't save enabled");

        utils::save_param("max_pool_percent", self.max_pool_percent)
            .expect("Couldn't save max_pool_percent");

        utils::save_param("same_filled_pages_enabled", self.same_filled_pages_enabled)
            .expect("Couldn't save same_filled_pages_enabled");

        utils::save_param("zpool", self.zpool).expect("Couldn't save zpool");
    }
}
