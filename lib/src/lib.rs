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
pub mod authentication;
pub mod balancer;
mod common;
pub mod config;
pub mod error;
mod locator;
pub mod pipeline;
mod service;
mod state;
pub mod templates;

//Expose consumable APIs
pub use app::Katalyst;
