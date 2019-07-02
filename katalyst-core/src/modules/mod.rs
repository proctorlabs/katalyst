/*!
Module traits and built in modules
*/

mod authentication;
mod authorization;
mod def;
mod handlers;
mod plugins;
mod result;

use crate::prelude::*;
use std::{collections::HashMap, sync::Arc};

pub(crate) mod balancer;
pub(crate) mod cache;
pub use cache::CachedObject;
pub use def::*;
pub(crate) use result::*;

bind_katalyst!(
    @ handlers::FileServerModule,
    handlers::HostModule,
    authentication::AlwaysAuthenticator,
    authentication::NeverAuthenticator,
    authentication::HttpAuthenticatorBuilder,
    authentication::WhitelistBuilder,
    plugins::ContentPlugin,
    cache::DefaultCacheHandler,
    cache::MemoryCacheBuilder,
    balancer::LeastConnectionBalancerBuilder,
    balancer::RandomBalancerBuilder,
    balancer::RoundRobinBalancerBuilder
);
