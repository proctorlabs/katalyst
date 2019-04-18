use crate::app::KatalystEngine;
use crate::authentication::KatalystAuthenticationInfo;
use crate::prelude::*;
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

#[derive(Debug)]
pub struct Detail {
    pub remote_ip: String,
    pub url: url::Url,
    pub matched_route: Option<Arc<Route>>,
    pub captures: Option<HashMap<String, String>>,
    pub authentication: Option<KatalystAuthenticationInfo>,
    pub balancer_lease: Option<Arc<String>>,
}

#[derive(Debug)]
pub struct Timestamps {
    pub started: Instant,
    pub completed: Option<Instant>,
}

#[derive(Debug)]
pub struct Context {
    pub upstream: RequestResponse,
    pub downstream: RequestResponse,
    pub detail: Detail,
    pub timestamps: Timestamps,
    pub engine: Arc<KatalystEngine>,
}

#[derive(Debug)]
pub struct ContextLock {
    pub detail: Detail,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            upstream: RequestResponse::default(),
            downstream: RequestResponse::default(),
            detail: Detail {
                remote_ip: String::default(),
                url: url::Url::parse("http://localhost/").unwrap(),
                matched_route: None,
                captures: None,
                authentication: None,
                balancer_lease: None,
            },
            timestamps: Timestamps {
                started: Instant::now(),
                completed: None,
            },
            engine: Arc::default(),
        }
    }
}

impl Context {
    pub fn new(
        request: Request<Body>,
        engine: Arc<KatalystEngine>,
        remote_addr: SocketAddr,
    ) -> Self {
        let mut path = String::new();
        let uri = request.uri();
        path.push_str(&uri.scheme_str().unwrap_or("http"));
        path.push_str("://");
        path.push_str(&uri.host().unwrap_or("localhost"));
        path.push_str(&uri.to_string());
        Context {
            upstream: RequestResponse {
                request: Some(request),
                response: None,
            },
            downstream: RequestResponse::default(),
            detail: Detail {
                remote_ip: remote_addr.ip().to_string(),
                url: url::Url::parse(&path).unwrap(),
                matched_route: None,
                captures: None,
                authentication: None,
                balancer_lease: None,
            },
            timestamps: Timestamps {
                started: Instant::now(),
                completed: None,
            },
            engine,
        }
    }

    pub fn lock(self) -> ContextLock {
        ContextLock {
            detail: self.detail,
        }
    }
}

impl Detail {
    pub fn route(&self) -> Result<&Arc<Route>, RequestFailure> {
        self.matched_route
            .as_ref()
            .ok_or_else(|| RequestFailure::Internal)
    }
}

impl RequestResponse {
    pub fn request(&self) -> Result<&Request<Body>, RequestFailure> {
        Ok(self
            .request
            .as_ref()
            .ok_or_else(|| RequestFailure::Internal)?)
    }

    pub fn response(&self) -> Result<&Response<Body>, RequestFailure> {
        Ok(self
            .response
            .as_ref()
            .ok_or_else(|| RequestFailure::Internal)?)
    }
}
