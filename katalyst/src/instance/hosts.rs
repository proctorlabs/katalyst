use crate::modules::{balancer::default_balancer, *};
use std::sync::Arc;

#[derive(Debug)]
pub struct Hosts {
    pub servers: Arc<dyn LoadBalancerModule>,
}

impl Default for Hosts {
    fn default() -> Self {
        Hosts { servers: default_balancer() }
    }
}
