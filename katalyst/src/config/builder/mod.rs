mod module;
mod path;
mod routes;
mod service;

pub use module::ModuleBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;
pub use service::InterfaceBuilder;
pub use service::ServiceBuilder;

use crate::{app::Katalyst, error::GatewayError, instance::*, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/// A configuration builder for an instance of Katalyst
pub trait Builder<T> {
    /// Build an instance configuration using the supplied base Katalyst instance
    fn build(&self, engine: Arc<Katalyst>) -> Result<T>;
}

/// The base builder for building a new Katalyst Instance
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    hosts: HashMap<String, ModuleBuilder<LoadBalancer>>,
    routes: Vec<RouteBuilder>,
    service: ServiceBuilder,
}

impl Builder<Instance> for KatalystBuilder {
    fn build(&self, engine: Arc<Katalyst>) -> Result<Instance> {
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
