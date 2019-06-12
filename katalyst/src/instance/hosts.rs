use crate::modules::{balancer::default_balancer, *};
use std::sync::Arc;

/// This is the directory of hosts/load balancers attacyed to this instance
#[derive(Debug)]
pub struct Hosts {
    /// The actual directory
    pub servers: Arc<dyn LoadBalancerModule>,
}

impl Default for Hosts {
    fn default() -> Self {
        Hosts { servers: default_balancer() }
    }
}
