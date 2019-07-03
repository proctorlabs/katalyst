/*!
This is the base module for the Katalyst API Gateway library.
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

#[macro_use]
pub mod extensions;

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
pub use extensions::{get_expression, get_module};
pub use katalyst_macros::ExpressionBinding;
pub use unstructured::Document;

katalyst_link! {
    modules: {
        modules::handlers::FileServerModule,
        modules::handlers::HostModule,
        modules::authentication::AlwaysAuthenticator,
        modules::authentication::NeverAuthenticator,
        modules::authentication::HttpAuthenticatorBuilder,
        modules::authentication::WhitelistBuilder,
        modules::plugins::ContentPlugin,
        modules::cache::DefaultCacheHandler,
        modules::cache::MemoryCacheBuilder,
        modules::balancer::LeastConnectionBalancerBuilder,
        modules::balancer::RandomBalancerBuilder,
        modules::balancer::RoundRobinBalancerBuilder
    }
}

katalyst_link! {
    expressions: {
        expression::Sys,
        expression::Http,
        expression::Auth,
        expression::Url,
        expression::Content,
        expression::Encode,
        expression::Decode
    }
}
