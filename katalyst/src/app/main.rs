mod opt;

use katalyst::Katalyst;
use opt::{Command, Opt};

fn main() -> Result<(), String> {
    let opts = Opt::new();
    let command = opts.command.as_ref().unwrap_or(&Command::Run);
    match command {
        Command::Run => start(&opts)?,
    };
    Ok(())
}

fn start(opts: &Opt) -> Result<(), String> {
    simple_logger::init_with_level(opts.log_level).map_err(|e| format!("{}", e))?;
    Katalyst::start(&opts.config).map_err(|e| format!("{}", e))?;
    Ok(())
}
