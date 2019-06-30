/*!
This module contains the request context information used by modules and the request
processing pipeline.
*/
mod auth;
mod data;
mod matched;
mod requests;

use crate::{
    instance::Route,
    prelude::*,
    util::{LockedResource, Resource},
};
use data::ContextData;
use hyper::{Body, Request};
use parking_lot::Mutex;
use std::{any::Any, net::SocketAddr, time::Instant};

pub use auth::Authentication;
pub use matched::Match;
pub use requests::*;

/// The base KatalystCore request context supplied to all modules and expressions
#[derive(Debug, Default, Clone)]
pub struct RequestContext {
    context: Arc<Context>,
}

impl std::ops::Deref for RequestContext {
    type Target = Arc<Context>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

/// The main request context
#[derive(Debug)]
pub struct Context {
    request: LockedResource<HttpRequest>,
    metadata: Arc<Metadata>,
    authentication: LockedResource<Authentication>,
    matched: LockedResource<Match>,
    data: Mutex<ContextData>,
    katalyst: Katalyst,
}

/// Metadata for this request context
#[derive(Debug)]
pub struct Metadata {
    /// Requeset processing start time
    pub started: Instant,
    /// Remote IP address of the client
    pub remote_ip: String,
    /// The parsed URL for this request
    pub url: url::Url,
    /// Holds the current load balancer lease for this request
    pub balancer_lease: Mutex<Option<Arc<String>>>,
}

impl Default for Context {
    fn default() -> Self {
        *Box::new(Context {
            request: LockedResource::new(HttpRequest::Empty),
            metadata: Arc::new(Metadata {
                remote_ip: String::default(),
                url: url::Url::parse("http://localhost/").unwrap(),
                balancer_lease: Mutex::new(None),
                started: Instant::now(),
            }),
            matched: LockedResource::new(Match::Unmatched),
            authentication: LockedResource::new(Authentication::Anonymous),
            data: Mutex::new(ContextData::default()),
            katalyst: Katalyst::new().unwrap(),
        })
    }
}

impl RequestContext {
    /// Create a new RequestContext with the supplied arguments
    pub fn new(request: Request<Body>, katalyst: Katalyst, remote_addr: SocketAddr) -> Self {
        let uri = request.uri();
        let path = format!(
            "{scheme}://{host}{path}",
            scheme = &uri.scheme_str().unwrap_or("http"),
            host = &uri.host().unwrap_or("localhost"),
            path = &uri
        );
        RequestContext {
            context: Arc::new(Context {
                request: LockedResource::new(HttpRequest::new(request)),
                metadata: Arc::new(Metadata {
                    remote_ip: remote_addr.ip().to_string(),
                    url: url::Url::parse(&path).unwrap(),
                    balancer_lease: Mutex::new(None),
                    started: Instant::now(),
                }),
                matched: LockedResource::new(Match::Unmatched),
                authentication: LockedResource::new(Authentication::Anonymous),
                data: Mutex::new(ContextData::default()),
                katalyst: katalyst.clone(),
            }),
        }
    }

    /// Get the base katalyst instance associated with this request
    pub fn katalyst(&self) -> Result<Katalyst> {
        Ok(self.katalyst.clone())
    }

    /// This request's metadata
    pub fn metadata(&self) -> Result<Arc<Metadata>> {
        Ok(self.metadata.clone())
    }

    /// Get authentication state of this request
    pub fn get_authentication(&self) -> Result<Resource<Authentication>> {
        Ok(self.authentication.get())
    }

    /// Change the authentication state for this request
    pub fn set_authentication(&self, info: Authentication) -> Result<()> {
        self.authentication.set(info);
        Ok(())
    }

    /// Get the match for this request
    pub fn get_match(&self) -> Result<Resource<Match>> {
        Ok(self.matched.get())
    }

    /// Get the matched route for this request
    pub fn get_route(&self) -> Result<Arc<Route>> {
        let resource = self.get_match()?;
        Ok(resource.route()?.clone())
    }

    /// Set the match for this request
    pub fn set_match(&self, matched: Match) -> Result<()> {
        self.matched.set(matched);
        Ok(())
    }

    /// Get custom extension data of type T
    pub fn get_extension<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.data.lock().get().ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, "Attempted to retrieve a module that does not exist!"))
    }

    /// Set custom extension data of type T
    pub fn set_extension<T: Any + Send + Sync>(&self, data: T) -> Result<()> {
        self.data.lock().set(data);
        Ok(())
    }
}
