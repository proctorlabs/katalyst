mod authenticator;
mod downstream;
mod listener;
mod routes;

pub use authenticator::AuthenticatorBuilder;
pub use downstream::DownstreamBuilder;
pub use listener::ListenerBuilder;
pub use routes::RouteBuilder;
pub use crate::state::*;

use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::KatalystState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct KatalystBuilder {
    routes: Vec<RouteBuilder>,
    listener: ListenerBuilder,
}

impl KatalystBuilder {
    pub fn build(&self, engine: Arc<KatalystEngine>) -> Result<KatalystState, KatalystError> {
        //build routes...
        let mut all_routes = vec![];
        for route in self.routes.iter() {
            all_routes.push(Arc::new(route.build(engine.clone())?));
        }

        let listener = self.listener.build(engine.clone())?;
        //final result
        Ok(KatalystState {
            routes: all_routes,
            listener: listener,
        })
    }
}
