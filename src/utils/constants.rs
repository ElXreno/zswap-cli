/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub const CONFIG_PATH: &str = "/etc/zswap-cli.conf";
pub const ZSWAP_BASEPATH: &str = "/sys/module/zswap/parameters";
pub const ZSWAP_DEBUG_BASEPATH: &str = "/sys/kernel/debug/zswap";
pub const PARAM_LIST: [&str; 6] = [
    "accept_threshold_percent",
    "compressor",
    "enabled",
    "max_pool_percent",
    "same_filled_pages_enabled",
    "zpool",
];
