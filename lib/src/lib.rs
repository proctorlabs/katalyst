#[macro_use]
extern crate log;

mod app;
mod config;
mod pipeline;
mod service;

pub use app::Katalyst;

/// This is the primary entrypoint for the API Gateway.
/// config_file must be the path (relative or absolute) to a YAML or JSON configuration file.
pub fn start_katalyst(config_file: &str) {
    let app = Katalyst::new();
    app.load(config_file);
    app.run();
}
