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

#[macro_use]
extern crate katalyst_macros;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

//Load mods
mod app;
pub mod balancer;
pub mod config;
pub mod context;
pub mod error;
pub mod expression;
mod instance;
mod locator;
pub mod modules;
pub mod pipeline;
pub mod prelude;
#[macro_use]
pub mod macros;
mod service;

//Expose consumable APIs
pub use app::Katalyst;
