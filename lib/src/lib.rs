//! # Katalyst
//!
//! Katalyst is a fast, simple, and efficient API Gateway.
//!
//! Katalyst can be used as a library in other rust projects (FFI for other languages planned)
//! or as a standalone application.

//Define package-wide macro providers
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

//Load mods
mod app;
mod authentication;
mod balancer;
mod common;
mod config;
mod error;
mod locator;
mod pipeline;
mod service;
mod state;
mod templates;

//Expose consumable APIs
pub use app::Katalyst;
pub use error::KatalystError;
pub use templates::KatalystTemplatePlaceholder;
pub use templates::KatalystTemplateProvider;
pub use templates::Providers;
