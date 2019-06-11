#[macro_use]
extern crate log;

mod cli;

use cli::{Args, Command};
use katalyst::Katalyst;

fn main() {
    ::std::process::exit(match start() {
        Err(e) => {
            error!("Could not start services. {}", e);
            1
        }
        Ok(_) => 0,
    })
}

fn start() -> Result<(), String> {
    let args = Args::new();
    simple_logger::init_with_level(args.log_level).map_err(|e| format!("{}", e))?;
    match args.command.as_ref().unwrap_or(&Command::Run) {
        Command::Run => {
            Katalyst::start(&args.config).map_err(|e| format!("{}", e))?;
            Ok(())
        }
    }
}
