/*!
Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a library.

# Features

Katalyst is still an experimental work in progress. Please see the [Roadmap](ROADMAP.md)
to see expected features.

Current features include:

* Simple YAML/JSON Gateway configuration
* Sophisticated regex routing
* API hooks for authentication modules
* Load balancing with Round Robin/Least Connection/Random algorithms
* Configurable service locator allowing for internal functionality to be overridden
* Flexible templating for value replacement in downstream requests
*/

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
pub mod expression;
mod locator;
pub mod pipeline;
mod service;
mod state;

//Expose consumable APIs
pub use app::Katalyst;
