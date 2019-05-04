use std::net::SocketAddr;

#[derive(Debug)]
pub struct Service {
    pub interface: SocketAddr,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            interface: "0.0.0.0:8080".parse().unwrap(),
        }
    }
}
