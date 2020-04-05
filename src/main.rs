#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, AppSettings, Arg, SubCommand};
use env_logger::Env;
use std::io::Write;
use std::process::exit;

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
                .arg(
                    Arg::with_name("use-config").long("use-config").help(
                        format!(
                            "When provides will be used config file from {}",
                            utils::constants::CONFIG_PATH
                        )
                        .as_str(),
                    ),
                ),
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
        Some("set") => {
            if let Some(ref matches) = matches.subcommand_matches("set") {
                debug!("Matched set subcommand");

                if !utils::is_root() {
                    error!("You are not a root user!");
                    exit(1);
                }

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