mod module;
mod path;
mod routes;
mod service;

pub use module::ModuleBuilder;
pub use path::PathBuilder;
pub use routes::RouteBuilder;
pub use service::InterfaceBuilder;
pub use service::ServiceBuilder;

use crate::{error::GatewayError, instance::*, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/// A configuration builder for an instance of KatalystCore
pub trait Builder<T> {
    /// Build an instance configuration using the supplied base KatalystCore instance
    fn build(&self) -> Result<T>;
}

/// The base builder for building a new KatalystCore Instance
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystCoreBuilder {
    hosts: HashMap<String, ModuleBuilder<LoadBalancer>>,
    routes: Vec<RouteBuilder>,
    service: ServiceBuilder,
}

impl Builder<Instance> for KatalystCoreBuilder {
    fn build(&self) -> Result<Instance> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build()?));
        }

        //build hosts...
        let mut hosts: HashMap<String, Hosts> = HashMap::new();
        for (k, v) in self.hosts.iter() {
            hosts
                .insert(k.to_string(), Hosts { servers: module_unwrap!(LoadBalancer, v.build()?) });
        }

        //final result
        Ok(Instance { hosts, routes: all_routes, service: self.service.build()? })
    }
}
