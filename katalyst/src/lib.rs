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

//#![warn(missing_docs)]
#![recursion_limit = "128"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_more;

mod app;
mod instance;
mod server;
mod util;

pub(crate) mod parser;

pub mod config;
pub mod context;
pub mod error;
pub mod expression;
pub mod modules;
#[macro_use]
pub mod prelude;

pub use app::Katalyst;
pub use katalyst_macros::ExpressionBinding;
