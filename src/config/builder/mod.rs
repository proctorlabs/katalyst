mod routes;

use crate::config::{Gateway, Listener};
pub use routes::RouteBuilder;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GatewayBuilder<'a> {
    pub routes: RefCell<Vec<RouteBuilder<'a>>>,
}

impl<'a> GatewayBuilder<'a> {
    pub fn build(&mut self) -> Gateway {
        println!("{:?}", self);
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.borrow().iter() {
            all_routes.push(route.clone().build());
        }

        //final result
        Gateway {
            routes: all_routes,
            listener: Listener {},
        }
    }
}
