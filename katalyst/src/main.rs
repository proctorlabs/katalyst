/*!
Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a library.

# Features

Katalyst is still an experimental work in progress. Please see the [Features](FEATURES.md)
list to see expected features.

Current features include:

* Simple YAML/JSON Gateway configuration
* Sophisticated regex routing
* API hooks for authentication modules
* Load balancing with Round Robin/Least Connection/Random algorithms
* Configurable service locator allowing for internal functionality to be overridden
* Flexible templating for value replacement in downstream requests
*/

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
