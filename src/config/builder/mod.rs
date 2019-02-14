mod listener;
mod routes;

use crate::config::Gateway;
use listener::*;
use routes::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GatewayBuilder<'a> {
    routes: RefCell<Vec<RouteBuilder<'a>>>,
    listener: RefCell<ListenerBuilder>,
}

impl<'a> GatewayBuilder<'a> {
    pub fn build(&mut self) -> Gateway {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.borrow().iter() {
            all_routes.push(route.clone().build());
        }

        let listener = self.listener.get_mut().build();
        //final result
        Gateway {
            routes: all_routes,
            listener: listener,
        }
    }
}
