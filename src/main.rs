#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, AppSettings, Arg, SubCommand};
use env_logger::Env;
use std::io::Write;

mod structs;
mod utils;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    trace!("Parsing arguments...");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("info").about("Displays current parameters"))
        .subcommand(
            SubCommand::with_name("stats")
                .about("Displays current zswap stats")
                .arg(
                    Arg::with_name("display-all")
                        .long("all")
                        .short("a")
                        .help("Displays all debug variables"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets configuration")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("accept_threshold_percent")
                        .long("accept-threshold-percent")
                        .help("Accept threshold percent")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(
                    Arg::with_name("compressor")
                        .long("compressor")
                        .help("Algorithm for compression")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(
                    Arg::with_name("enabled")
                        .long("enabled")
                        .help("Enable zswap")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(
                    Arg::with_name("max_pool_percent")
                        .long("max-pool-percent")
                        .help("Max pool percent")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(
                    Arg::with_name("same_filled_pages_enabled")
                        .long("same-filled-pages-enabled")
                        .help("Enable same filled pages deduplication")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(
                    Arg::with_name("zpool")
                        .long("zpool")
                        .help("Zpool type")
                        .takes_value(true)
                        .value_name("VALUE"),
                )
                .arg(Arg::with_name("use-config").long("use-config").help(
                    format!("Use config file from {}", utils::constants::CONFIG_PATH).as_str(),
                )),
        )
        .get_matches();

    debug!("Matching subcommand...");
    match matches.subcommand_name() {
        Some("info") => {
            debug!("Matched info subcommand");

            let sys_params = structs::ZswapParams::load_sys_params();

            for sys_param in sys_params.params {
                info!(
                    "{}: {}",
                    sys_param.name,
                    sys_param.sys_value.unwrap_or("NaN".to_string())
                );
            }
        }
        Some("stats") => {
            if let Some(ref matches) = matches.subcommand_matches("stats") {
                debug!("Matched stats subcommand");

                utils::check_root();

                let debug_params = structs::ZswapDebugParams::load_sys_params();

                if matches.is_present("display-all") {
                    for debug_param in debug_params.params {
                        match debug_param.name.as_str() {
                            "same_filled_pages" | "stored_pages" => {
                                let value = debug_param.sys_value.unwrap_or(0);
                                // TODO: Get page size instead of hardcoded value
                                let float_value = (value as usize * utils::get_page_size()) as f64
                                    / 1024.0
                                    / 1024.0;

                                info!("{}: {:.2} MB", debug_param.name, float_value);
                            }
                            "pool_total_size" => {
                                let value = debug_param.sys_value.unwrap_or(0);
                                let float_value = value as f64 / 1024.0 / 1024.0;

                                info!("{}: {:.2} MB", debug_param.name, float_value)
                            }
                            _ => info!(
                                "{}: {}",
                                debug_param.name,
                                debug_param.sys_value.unwrap_or(0)
                            ),
                        }
                    }
                } else {
                    let pool_size = debug_params
                        .params
                        .iter()
                        .find(|x| x.name == String::from("pool_total_size"))
                        .unwrap_or(&structs::ZswapDebugParam::default())
                        .sys_value
                        .unwrap_or(0);

                    let pages_size = {
                        let pages_size = debug_params
                            .params
                            .iter()
                            .find(|x| x.name == String::from("stored_pages"))
                            .unwrap_or(&structs::ZswapDebugParam::default())
                            .sys_value
                            .unwrap_or(0);

                        (pages_size as usize * utils::get_page_size()) as i64
                    };

                    let mem_info = linux_stats::meminfo().unwrap_or_default();

                    let consumed_size = (mem_info.swap_total - mem_info.swap_free) as f64 / 1024.0;

                    let compression_ratio = pages_size as f64 / pool_size as f64;

                    let pool_size = pool_size as f64 / 1024.0 / 1024.0;
                    let pages_size = pages_size as f64 / 1024.0 / 1024.0;

                    info!(
                        "Pool size: {:.2} MB | Stored pages: {:.2} MB | Consumed {:.1} MB | Compression ratio: {:.1}",
                        pool_size, pages_size, consumed_size, compression_ratio
                    );
                }
            }
        }
        Some("set") => {
            if let Some(ref matches) = matches.subcommand_matches("set") {
                debug!("Matched set subcommand");

                utils::check_root();

                debug!("Getting params from matches...");
                let mut params = structs::ZswapParams::load_sys_params();

                if matches.is_present("use-config") {
                    params.load_params_from_config();
                }
                params.load_params_from_matches(&matches);
                params.save();
            }
        }
        _ => {
            debug!("Matched None o_O");
        }
    }
}
