mod auth;
mod data;
mod matched;
mod requests;

use crate::{
    app::Katalyst,
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

#[derive(Debug)]
pub struct Context {
    request: LockedResource<HttpRequest>,
    metadata: Arc<Metadata>,
    authentication: LockedResource<Authentication>,
    matched: LockedResource<Match>,
    data: Mutex<ContextData>,
    katalyst: Arc<Katalyst>,
}

#[derive(Debug)]
pub struct Metadata {
    pub started: Instant,
    pub remote_ip: String,
    pub url: url::Url,
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
            katalyst: Arc::new(Katalyst::default()),
        })
    }
}

impl RequestContext {
    pub fn new(request: Request<Body>, katalyst: Arc<Katalyst>, remote_addr: SocketAddr) -> Self {
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

    pub fn katalyst(&self) -> Result<Arc<Katalyst>> {
        Ok(self.katalyst.clone())
    }

    pub fn metadata(&self) -> Result<Arc<Metadata>> {
        Ok(self.metadata.clone())
    }

    pub fn get_authentication(&self) -> Result<Resource<Authentication>> {
        Ok(self.authentication.get())
    }

    pub fn set_authentication(&self, info: Authentication) -> Result<()> {
        self.authentication.set(info);
        Ok(())
    }

    pub fn get_match(&self) -> Result<Resource<Match>> {
        Ok(self.matched.get())
    }

    pub fn get_route(&self) -> Result<Arc<Route>> {
        let resource = self.get_match()?;
        Ok(resource.route()?.clone())
    }

    pub fn set_match(&self, matched: Match) -> Result<()> {
        self.matched.set(matched);
        Ok(())
    }

    pub fn get_extension<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.data.lock().get().ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, "Attempted to retrieve a module that does not exist!"))
    }

    pub fn set_extension<T: Any + Send + Sync>(&self, data: T) -> Result<()> {
        self.data.lock().set(data);
        Ok(())
    }
}
