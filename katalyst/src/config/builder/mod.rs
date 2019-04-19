mod authenticator;
mod handler;
mod hosts;
mod listener;
mod path;
mod routes;

pub use crate::instance::*;
pub use authenticator::AuthenticatorBuilder;
pub use handler::HandlerBuilder;
pub use hosts::HostsBuilder;
pub use listener::ListenerBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;

use crate::app::KatalystEngine;
use crate::error::ConfigurationFailure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Builder<T> {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<T, ConfigurationFailure>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    hosts: HashMap<String, HostsBuilder>,
    routes: Vec<RouteBuilder>,
    listener: ListenerBuilder,
}

impl KatalystBuilder {
    pub fn build(self, engine: Arc<KatalystEngine>) -> Result<KatalystState, ConfigurationFailure> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(engine.clone())?));
        }

        //final result
        Ok(KatalystState {
            hosts: self.hosts.build(engine.clone())?,
            routes: all_routes,
            listener: self.listener.build(engine.clone())?,
        })
    }
}
