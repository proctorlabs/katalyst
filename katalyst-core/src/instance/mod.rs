/*!
Instance provides details for the current running state of Katalyst.
*/

mod hosts;
mod route;
mod service;

pub use hosts::Hosts;
pub use route::Route;
pub use service::{Interface, Service};
use std::{collections::HashMap, sync::Arc};

/// The primary Katalyst instance configuration
#[derive(Debug, Default)]
pub struct Instance {
    /// This is the directory of hosts/load balancers attacyed to this instance
    pub hosts: HashMap<String, Hosts>,
    /// The routes associated with this instance
    pub routes: Vec<Arc<Route>>,
    /// Base service metadata
    pub service: Service,
}
