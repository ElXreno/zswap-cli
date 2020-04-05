pub const CONFIG_PATH: &str = "/etc/zswap-cli.conf";
pub const ZSWAP_BASEPATH: &str = "/sys/module/zswap/parameters";
pub const PARAM_LIST: [&str; 6] = [
    "accept_threshold_percent",
    "compressor",
    "enabled",
    "max_pool_percent",
    "same_filled_pages_enabled",
    "zpool",
];
