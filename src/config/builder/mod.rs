mod routes;

use crate::config::{Gateway, Listener};
pub use routes::RouteBuilder;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct GatewayBuilder<'a> {
    pub routes: RefCell<Vec<RouteBuilder<'a>>>,
    pub listener: RefCell<Option<Listener>>,
}

impl<'a> GatewayBuilder<'a> {
    pub fn new() -> Self {
        GatewayBuilder {
            routes: RefCell::new(vec![]),
            listener: RefCell::new(None),
        }
    }

    pub fn push_routes(&mut self, routes: &mut Vec<RouteBuilder<'a>>) {
        self.routes.get_mut().append(routes);
    }

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
