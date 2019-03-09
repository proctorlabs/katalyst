use crate::options::{parse_log_level, KatalystCommand, KatalystOptions};
use clap::{App, Arg, SubCommand};

pub fn parse_cli() -> KatalystOptions {
    let matches = App::new("Katalyst")
        .version(crate_version!())
        .author("Phil Proctor <philliptproctor@gmail.com>")
        .about("Katalyst is a high performance, low memory API Gateway.")
        .bin_name("katalyst")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .env("KATALYST_CONFIG")
                .value_name("FILE")
                .help("Configuration file to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log-level")
                .env("KATALYST_LOGLEVEL")
                .value_name("LOGLEVEL")
                .default_value("debug")
                .takes_value(true)
                .possible_values(&["trace", "debug", "info", "warn", "error"])
                .help("Sets the log level to use"),
        )
        .subcommand(SubCommand::with_name("start").about("Starts the API Gateway (default)"))
        .get_matches();
    KatalystOptions {
        command: KatalystCommand::parse_command(matches.subcommand_name().unwrap_or("start")),
        config_file: matches.value_of("config").unwrap().to_string(),
        log_level: parse_log_level(matches.value_of("log").unwrap()),
    }
}
