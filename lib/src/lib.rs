//Define package-wide macro providers
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

//Load mods
mod app;
mod config;
mod pipeline;
mod service;
mod templates;

//Expose consumable APIs
pub use app::Katalyst;
pub use templates::KatalystTemplatePlaceholder;
pub use templates::KatalystTemplateProvider;
pub use templates::Providers;

/// This is the primary entrypoint for the API Gateway.
/// config_file must be the path (relative or absolute) to a YAML or JSON configuration file.
pub fn start_katalyst(config_file: &str) -> Katalyst {
    let app = Katalyst::default();
    app.load(config_file);
    app.run();
    app
}
