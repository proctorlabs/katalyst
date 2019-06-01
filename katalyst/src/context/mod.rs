mod auth;
mod data;
mod requests;

use crate::{app::Katalyst, instance::Route, prelude::*};
pub use auth::KatalystAuthenticationInfo;
use data::ContextData;
use hyper::{Body, Request};
pub use requests::*;
use std::{any::Any, collections::HashMap, net::SocketAddr, sync::Arc, time::Instant};

#[derive(Debug)]
pub struct Context {
    pub request: HttpRequest,
    pub metadata: Metadata,
    pub katalyst: Arc<Katalyst>,
    state: RequestState,
    data: ContextData,
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
    pub balancer_lease: Option<Arc<String>>,
}

#[derive(Debug)]
pub struct MatchInfo {
    pub route: Arc<Route>,
    pub captures: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AuthInfo {
    pub matched: MatchInfo,
    pub detail: KatalystAuthenticationInfo,
}

impl Default for Context {
    fn default() -> Self {
        *Box::new(Context {
            request: HttpRequest::Empty,
            metadata: Metadata {
                remote_ip: String::default(),
                url: url::Url::parse("http://localhost/").unwrap(),
                balancer_lease: None,
                started: Instant::now(),
            },
            state: RequestState::New,
            katalyst: Arc::default(),
            data: ContextData::default(),
        })
    }
}

impl Context {
    pub fn into_error(self, error: GatewayError) -> ModuleError {
        ModuleError { error, context: self }
    }

    pub fn get_matched(&self) -> Result<&MatchInfo> {
        match &self.state {
            RequestState::Matched(ref m) => Ok(m),
            RequestState::Authenticated(ref a) => Ok(&a.matched),
            _ => Err(GatewayError::StateUnavailable),
        }
    }

    pub fn set_match(&mut self, matched: MatchInfo) -> Result<()> {
        match &self.state {
            RequestState::New => {
                self.state = RequestState::Matched(matched);
                Ok(())
            }
            _ => Err(GatewayError::StateUpdateFailure),
        }
    }

    pub fn get_authenticated(&self) -> Result<&AuthInfo> {
        match &self.state {
            RequestState::Authenticated(ref a) => Ok(a),
            _ => Err(GatewayError::StateUnavailable),
        }
    }

    pub fn set_authenticated(
        mut self,
        info: KatalystAuthenticationInfo,
    ) -> std::result::Result<Self, ModuleError> {
        match self.state {
            RequestState::Matched(m) => {
                self.state = RequestState::Authenticated(AuthInfo { matched: m, detail: info })
            }
            _ => return Err(self.into_error(GatewayError::StateUpdateFailure)),
        };
        Ok(self)
    }

    pub fn get_extension_data<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.data.get().ok_or_else(|| GatewayError::InternalServerError)
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
        *Box::new(Context {
            request: HttpRequest::new(request),
            metadata: Metadata {
                remote_ip: remote_addr.ip().to_string(),
                url: url::Url::parse(&path).unwrap(),
                balancer_lease: None,
                started: Instant::now(),
            },
            state: RequestState::New,
            katalyst,
            data: ContextData::default(),
        })
    }
}
