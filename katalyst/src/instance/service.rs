use crate::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
pub struct Service {
    pub interface: SocketAddr,
    pub cache: Arc<CacheProviderModule>,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            interface: "0.0.0.0:8080".parse().unwrap(),
            cache: cache::default_cache(),
        }
    }
}
