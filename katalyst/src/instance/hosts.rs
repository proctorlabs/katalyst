use crate::balancer::{default_balancer, KatalystBalancer};
use std::sync::Arc;

#[derive(Debug)]
pub struct Hosts {
    pub servers: Arc<KatalystBalancer>,
}

impl Default for Hosts {
    fn default() -> Self {
        Hosts { servers: default_balancer() }
    }
}
