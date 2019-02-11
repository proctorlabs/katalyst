mod app;
mod config;
mod service;
use app::Katalyst;

pub fn katalyst(config_file: &str) {
    let app = Katalyst::new();
    app.load(config_file);
    app.run();
}
