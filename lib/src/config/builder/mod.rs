mod downstream;
mod listener;
mod routes;

use crate::state::KatalystState;
use crate::templates::Providers;
use listener::*;
use routes::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GatewayBuilder {
    routes: Vec<RouteBuilder>,
    listener: ListenerBuilder,
}

impl GatewayBuilder {
    pub fn build(&self, providers: &Providers) -> KatalystState {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(providers)));
        }

        let listener = self.listener.build(providers);
        //final result
        KatalystState {
            routes: all_routes,
            listener: listener,
        }
    }
}
