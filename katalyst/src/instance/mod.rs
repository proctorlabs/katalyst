mod hosts;
mod listener;
mod route;

pub use hosts::Hosts;
pub use listener::Listener;
pub use route::Route;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct KatalystState {
    pub hosts: HashMap<String, Hosts>,
    pub routes: Vec<Arc<Route>>,
    pub listener: Listener,
}
