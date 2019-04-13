use log::Level;

#[derive(Debug)]
pub struct KatalystOptions {
    pub command: KatalystCommand,
    pub config_file: String,
    pub log_level: Level,
}

#[derive(Debug)]
pub enum KatalystCommand {
    Start,
}

impl KatalystCommand {
    pub fn parse_command(command: &str) -> Self {
        match command.to_lowercase().as_str() {
            _ => KatalystCommand::Start,
        }
    }
}

pub fn parse_log_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "warn" => Level::Warn,
        "error" => Level::Error,
        "info" => Level::Info,
        "debug" => Level::Debug,
        _ => Level::Trace,
    }
}
