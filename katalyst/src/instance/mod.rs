/*!
Instance provides details for the current running state of Katalyst.
*/

mod hosts;
mod route;
mod service;

pub use hosts::Hosts;
pub use route::Route;
pub use service::Service;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Instance {
    pub hosts: HashMap<String, Hosts>,
    pub routes: Vec<Arc<Route>>,
    pub service: Service,
}
