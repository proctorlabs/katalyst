use crate::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
pub enum Interface {
    Http { addr: SocketAddr },
    Https { addr: SocketAddr, cert: String, key: String },
}

impl Default for Interface {
    fn default() -> Self {
        Interface::Http { addr: "0.0.0.0:8080".parse().unwrap() }
    }
}

#[derive(Debug)]
pub struct Service {
    pub interfaces: Vec<Interface>,
    pub cache: Arc<CacheProviderModule>,
}

impl Default for Service {
    fn default() -> Self {
        Service { interfaces: vec![], cache: cache::default_cache() }
    }
}
