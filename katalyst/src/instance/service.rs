use crate::prelude::*;
use std::{net::SocketAddr, sync::Arc};

/// This instance's interface description
#[derive(Debug)]
pub enum Interface {
    /// An HTTP interface
    Http {
        /// The binding address for this interface
        addr: SocketAddr,
    },
    /// An HTTPS interface
    Https {
        /// The binding address for this interface
        addr: SocketAddr,
        /// The certifacte path
        cert: String,
        /// The certificate key path
        key: String,
    },
}

impl Default for Interface {
    fn default() -> Self {
        Interface::Http { addr: "0.0.0.0:8080".parse().unwrap() }
    }
}

/// The API Gateway service metadata
#[derive(Debug)]
pub struct Service {
    /// Array of interfaces for this service
    pub interfaces: Vec<Interface>,
    /// The cache provider for this service
    pub cache: Arc<dyn CacheProviderModule + Send>,
}

impl Default for Service {
    fn default() -> Self {
        Service { interfaces: vec![], cache: cache::default_cache() }
    }
}
