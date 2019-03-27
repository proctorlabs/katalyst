#[macro_use]
extern crate clap;

mod cli;
mod options;
use katalyst::Katalyst;
use options::*;

fn main() {
    let opts = cli::parse_cli();
    match opts.command {
        KatalystCommand::Start => start(opts),
    }
}

fn start(opts: KatalystOptions) {
    simple_logger::init_with_level(opts.log_level).unwrap();
    Katalyst::start(&opts.config_file).unwrap();
}
