mod auth;
mod data;
mod requests;

use crate::{app::Katalyst, instance::Route, prelude::*};
pub use auth::KatalystAuthenticationInfo;
use data::ContextData;
use hyper::{Body, Request};
use parking_lot::Mutex;
pub use requests::*;
use std::{any::Any, collections::HashMap, net::SocketAddr, time::Instant};

#[derive(Debug, Default, Clone)]
pub struct ContextGuard {
    context: Arc<Context>,
    katalyst: Arc<Katalyst>,
}

impl ContextGuard {
    pub fn new(request: Request<Body>, katalyst: Arc<Katalyst>, remote_addr: SocketAddr) -> Self {
        let uri = request.uri();
        let path = format!(
            "{scheme}://{host}{path}",
            scheme = &uri.scheme_str().unwrap_or("http"),
            host = &uri.host().unwrap_or("localhost"),
            path = &uri
        );
        ContextGuard {
            context: Arc::new(Context {
                request: Arc::new(Mutex::new(HttpRequest::new(request))),
                metadata: Arc::new(Metadata {
                    remote_ip: remote_addr.ip().to_string(),
                    url: url::Url::parse(&path).unwrap(),
                    balancer_lease: Mutex::new(None),
                    started: Instant::now(),
                }),
                state: Arc::new(Mutex::new(RequestState::New)),
                data: Mutex::new(ContextData::default()),
            }),
            katalyst: katalyst.clone(),
        }
    }

    pub fn katalyst(&self) -> Result<Arc<Katalyst>> {
        Ok(self.katalyst.clone())
    }

    pub fn state(&self) -> Result<Arc<Mutex<RequestState>>> {
        Ok(self.context.state.clone())
    }

    pub fn metadata(&self) -> Result<Arc<Metadata>> {
        Ok(self.context.metadata.clone())
    }

    pub fn context(&self) -> Result<Arc<Context>> {
        Ok(self.context.clone())
    }

    pub fn get_authenticated(&self) -> Result<AuthInfo> {
        let state: &mut RequestState = &mut self.context.state.lock();
        match state {
            RequestState::Authenticated(ref a) => Ok(a.clone()),
            _ => Err(GatewayError::StateUnavailable),
        }
    }

    pub fn set_authenticated(&self, info: KatalystAuthenticationInfo) -> Result<()> {
        let state: &mut RequestState = &mut self.context.state.lock();
        match state {
            RequestState::Matched(m) => {
                *state = RequestState::Authenticated(AuthInfo { matched: m.clone(), detail: info })
            }
            _ => return Err(GatewayError::StateUpdateFailure),
        };
        Ok(())
    }

    pub fn get_matched(&self) -> Result<MatchInfo> {
        let state: &mut RequestState = &mut self.context.state.lock();
        match state {
            RequestState::Matched(ref m) => Ok(m.clone()),
            RequestState::Authenticated(ref a) => Ok(a.matched.clone()),
            _ => Err(GatewayError::StateUnavailable),
        }
    }

    pub fn set_match(&self, matched: MatchInfo) -> Result<()> {
        let state: &mut RequestState = &mut self.context.state.lock();
        match state {
            RequestState::New => {
                *state = RequestState::Matched(matched);
                Ok(())
            }
            _ => Err(GatewayError::StateUpdateFailure),
        }
    }

    pub fn get_extension_data<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.context.data.lock().get().ok_or_else(|| GatewayError::InternalServerError)
    }

    pub fn set_extension_data<T: Any + Send + Sync>(&self, data: T) -> Result<()> {
        self.context.data.lock().set(data);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Context {
    request: Arc<Mutex<HttpRequest>>,
    metadata: Arc<Metadata>,
    state: Arc<Mutex<RequestState>>,
    data: Mutex<ContextData>,
}

#[derive(Debug)]
pub enum RequestState {
    New,
    Matched(MatchInfo),
    Authenticated(AuthInfo),
    Response(HttpRequest),
}

#[derive(Debug)]
pub struct Metadata {
    pub started: Instant,
    pub remote_ip: String,
    pub url: url::Url,
    pub balancer_lease: Mutex<Option<Arc<String>>>,
}

#[derive(Debug, Clone)]
pub struct MatchInfo {
    pub route: Arc<Route>,
    pub captures: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub matched: MatchInfo,
    pub detail: KatalystAuthenticationInfo,
}

impl Default for Context {
    fn default() -> Self {
        *Box::new(Context {
            request: Arc::new(Mutex::new(HttpRequest::Empty)),
            metadata: Arc::new(Metadata {
                remote_ip: String::default(),
                url: url::Url::parse("http://localhost/").unwrap(),
                balancer_lease: Mutex::new(None),
                started: Instant::now(),
            }),
            state: Arc::new(Mutex::new(RequestState::New)),
            data: Mutex::new(ContextData::default()),
        })
    }
}
