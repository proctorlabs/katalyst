mod data;
mod requests;

use crate::app::Katalyst;
use crate::instance::Route;
use crate::prelude::*;
use data::ContextData;
use hyper::{Body, Request};
pub use requests::*;
use std::any::Any;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Default)]
pub struct KatalystAuthenticationInfo {
    claims: HashMap<String, Vec<String>>,
}

impl KatalystAuthenticationInfo {
    pub fn add_claim(&mut self, claim_type: String, claim_value: String) {
        if let Some(claims) = self.claims.get_mut(&claim_type) {
            claims.push(claim_value);
        } else {
            self.claims.insert(claim_type, vec![claim_value]);
        }
    }

    pub fn get_claim(&self, claim_type: String) -> String {
        match self.claims.get(&claim_type) {
            Some(c) => c[0].to_string(),
            None => String::default(),
        }
    }
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
    pub request: RequestContainer,
    pub response: ResponseContainer,
    pub detail: Detail,
    pub timestamps: Timestamps,
    pub katalyst: Arc<Katalyst>,
    data: ContextData,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            request: RequestContainer::Empty,
            response: ResponseContainer::Empty,
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
            katalyst: Arc::default(),
            data: ContextData::default(),
        }
    }
}

impl Context {
    pub fn get_extension_data<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.data
            .get()
            .ok_or_else(|| GatewayError::InternalServerError)
    }

    pub fn set_extension_data<T: Any + Send + Sync>(&mut self, data: T) {
        self.data.set(data)
    }

    pub fn new(request: Request<Body>, katalyst: Arc<Katalyst>, remote_addr: SocketAddr) -> Self {
        let uri = request.uri();
        let path = format!(
            "{scheme}://{host}{path}",
            scheme = &uri.scheme_str().unwrap_or("http"),
            host = &uri.host().unwrap_or("localhost"),
            path = &uri
        );
        Context {
            request: RequestContainer::new(request),
            response: ResponseContainer::Empty,
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
            katalyst,
            data: ContextData::default(),
        }
    }
}

impl Detail {
    pub fn route(&self) -> Result<&Arc<Route>> {
        self.matched_route
            .as_ref()
            .ok_or_else(|| GatewayError::InternalServerError)
    }
}
