mod hosts;
mod module;
mod path;
mod routes;
mod service;

pub use crate::instance::*;
pub use hosts::HostsBuilder;
pub use module::ModuleBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;
pub use service::ServiceBuilder;

use crate::app::Katalyst;
use crate::error::GatewayError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Builder<T> {
    fn build(&self, engine: Arc<Katalyst>) -> Result<T, GatewayError>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    hosts: HashMap<String, HostsBuilder>,
    routes: Vec<RouteBuilder>,
    service: ServiceBuilder,
}

impl KatalystBuilder {
    pub fn build(self, engine: Arc<Katalyst>) -> Result<Instance, GatewayError> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(engine.clone())?));
        }

        //final result
        Ok(Instance {
            hosts: self.hosts.build(engine.clone())?,
            routes: all_routes,
            service: self.service.build(engine.clone())?,
        })
    }
}
