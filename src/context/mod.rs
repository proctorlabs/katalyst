use crate::app::KatalystEngine;
use crate::authentication::KatalystAuthenticationInfo;
use crate::state::Route;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

#[derive(Default)]
pub struct RequestResponse {
    pub request: Option<Request<Body>>,
    pub response: Option<Response<Body>>,
}

#[derive(Default)]
pub struct RequestContext {
    pub matched_route: Option<Arc<Route>>,
    pub captures: Option<HashMap<String, String>>,
    pub timestamps: HashMap<String, Instant>,
    pub authentication: Option<KatalystAuthenticationInfo>,
    pub balancer_lease: Option<Arc<String>>,
}

pub struct Context {
    pub upstream: RequestResponse,
    pub downstream: RequestResponse,
    pub context: RequestContext,
    pub remote_addr: SocketAddr,
    pub engine: Arc<KatalystEngine>,
}

impl Context {
    pub fn new(request: Request<Body>, engine: Arc<KatalystEngine>, remote: SocketAddr) -> Self {
        let mut state = Context {
            upstream: RequestResponse::default(),
            downstream: RequestResponse::default(),
            context: RequestContext::default(),
            engine: engine,
            remote_addr: remote,
        };
        state.upstream.request = Some(request);
        state
    }
}
