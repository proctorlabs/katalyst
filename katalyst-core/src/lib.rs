/*!
This is the base module for the KatalystCore API Gateway library.
*/

#![warn(missing_docs)]
#![recursion_limit = "128"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_more;

#[macro_use]
pub mod prelude;

mod app;
mod client;
mod instance;
mod server;
mod util;

pub(crate) mod parser;

pub mod config;
pub mod context;
pub mod error;
pub mod expression;
pub mod modules;
pub use app::Katalyst;
pub use client::ProxyClient;
pub use katalyst_macros::ExpressionBinding;