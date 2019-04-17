mod authenticator;
mod handler;
mod hosts;
mod listener;
mod route;

use std::collections::HashMap;
use std::sync::Arc;

pub use authenticator::Authenticator;
pub use handler::Handler;
pub use hosts::Hosts;
pub use listener::Listener;
pub use route::Route;

#[derive(Debug, Default)]
pub struct KatalystState {
    pub hosts: HashMap<String, Hosts>,
    pub routes: Vec<Arc<Route>>,
    pub listener: Listener,
}
