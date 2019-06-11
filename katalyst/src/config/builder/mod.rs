mod module;
mod path;
mod routes;
mod service;

pub use crate::instance::*;
pub use module::ModuleBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;
pub use service::ServiceBuilder;

use crate::{app::Katalyst, error::GatewayError, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

pub trait Builder<T> {
    fn build(&self, engine: Arc<Katalyst>) -> Result<T>;
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    hosts: HashMap<String, ModuleBuilder<LoadBalancer>>,
    routes: Vec<RouteBuilder>,
    service: ServiceBuilder,
}

impl KatalystBuilder {
    pub fn build(self, engine: Arc<Katalyst>) -> Result<Instance> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(engine.clone())?));
        }

        //build hosts...
        let mut hosts: HashMap<String, Hosts> = HashMap::new();
        for (k, v) in self.hosts.iter() {
            hosts.insert(
                k.to_string(),
                Hosts { servers: module_unwrap!(LoadBalancer, v.build(engine.clone())?) },
            );
        }

        //final result
        Ok(Instance { hosts, routes: all_routes, service: self.service.build(engine.clone())? })
    }
}
