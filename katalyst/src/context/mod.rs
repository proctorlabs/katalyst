use crate::app::KatalystEngine;
use crate::authentication::KatalystAuthenticationInfo;
use crate::state::Route;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

#[derive(Default, Debug)]
pub struct RequestResponse {
    pub request: Option<Request<Body>>,
    pub response: Option<Response<Body>>,
}

#[derive(Default, Debug)]
pub struct Detail {
    pub matched_route: Option<Arc<Route>>,
    pub captures: Option<HashMap<String, String>>,
    pub timestamps: HashMap<String, Instant>,
    pub authentication: Option<KatalystAuthenticationInfo>,
    pub balancer_lease: Option<Arc<String>>,
}

#[derive(Debug)]
pub struct Context {
    pub upstream: RequestResponse,
    pub downstream: RequestResponse,
    pub detail: Detail,
    pub remote_addr: SocketAddr,
    pub engine: Arc<KatalystEngine>,
}

impl Context {
    pub fn new(
        request: Request<Body>,
        engine: Arc<KatalystEngine>,
        remote_addr: SocketAddr,
    ) -> Self {
        let mut state = Context {
            upstream: RequestResponse::default(),
            downstream: RequestResponse::default(),
            detail: Detail::default(),
            engine,
            remote_addr,
        };
        state.upstream.request = Some(request);
        state
    }

    pub fn lock(self) -> ContextLock {
        ContextLock {
            detail: self.detail,
            remote_addr: self.remote_addr,
        }
    }
}

#[derive(Debug)]
pub struct ContextLock {
    pub detail: Detail,
    pub remote_addr: SocketAddr,
}
