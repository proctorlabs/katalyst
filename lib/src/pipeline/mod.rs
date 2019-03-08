mod builder;
mod logger;
mod matcher;
mod sender;

use crate::app::HttpsClient;
use crate::config::{Gateway, Route};
use builder::Builder;
use futures::future::*;
use futures::Future;
use hyper::{Body, Request, Response, StatusCode};
use logger::Logger;
use matcher::Matcher;
use sender::Sender;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

pub struct PipelineState {
    pub upstream_request: Request<Body>,
    pub upstream_response: Response<Body>,
    pub downstream_request: Option<Request<Body>>,
    pub downstream_response: Option<Response<Body>>,
    pub timestamps: HashMap<String, Instant>,
    pub matched_route: Box<Option<Route>>,
    pub captures: Option<HashMap<String, String>>,
    pub client: Arc<HttpsClient>,
    pub remote_addr: SocketAddr,
}

impl PipelineState {
    fn new(request: Request<Body>, client: Arc<HttpsClient>, remote: SocketAddr) -> Self {
        PipelineState {
            upstream_request: request,
            upstream_response: Response::default(),
            downstream_request: None,
            downstream_response: None,
            matched_route: Box::new(None),
            timestamps: HashMap::new(),
            captures: None,
            client: client,
            remote_addr: remote,
        }
    }

    fn return_status(&mut self, status: StatusCode) {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = status;
        self.upstream_response = response;
    }
}

#[derive(Debug)]
pub enum PipelineError {
    Halted,
    Failed,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for PipelineError {}

pub type HyperResult = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub type PipelineResult = Box<Future<Item = PipelineState, Error = PipelineError> + Send>;

pub trait Pipeline: Send + Sync {
    fn name(&self) -> &'static str;

    fn process(&self, state: PipelineState, _config: &Gateway) -> PipelineResult {
        self.ok(state)
    }

    fn post(&self, _state: &PipelineState) {}

    fn error(&self, _state: &PipelineError) {}

    fn make(&self) -> Box<Pipeline + Send + Sync>;

    fn result(&self, res: Result<PipelineState, PipelineError>) -> PipelineResult {
        Box::new(result(res))
    }

    fn ok(&self, state: PipelineState) -> PipelineResult {
        Box::new(
            ok::<PipelineState, PipelineError>(state).then(|s| match &s {
                Ok(_) => {
                    //println!("post");
                    s
                }
                Err(_) => {
                    //println!("error");
                    s
                }
            }),
        )
    }

    fn fail(&self, error: PipelineError) -> PipelineResult {
        Box::new(err::<PipelineState, PipelineError>(error))
    }
}

pub struct PipelineRunner {
    pipelines: Arc<[Arc<Pipeline + Send + Sync>]>,
    client: Arc<HttpsClient>,
}

impl PipelineRunner {
    pub fn new(client: Arc<HttpsClient>) -> Self {
        let pipelines: Arc<[Arc<Pipeline + Send + Sync>]> = Arc::new([
            Arc::new(Logger {}),
            Arc::new(Matcher {}),
            Arc::new(Builder {}),
            Arc::new(Sender {}),
        ]);
        PipelineRunner {
            pipelines: pipelines,
            client: client,
        }
    }

    pub fn run(
        &self,
        remote_addr: SocketAddr,
        request: Request<Body>,
        inc_config: &Gateway,
    ) -> HyperResult {
        let config = Arc::new(inc_config.clone());
        let client = self.client.clone();
        let mut result: Box<Future<Item = PipelineState, Error = PipelineError> + Send> =
            Box::new(lazy(move || {
                ok::<PipelineState, PipelineError>(PipelineState::new(request, client, remote_addr))
            }));
        for pip in self.pipelines.iter() {
            let c = config.clone();
            let runner = pip.clone();
            result = Box::new(result.and_then(move |s| runner.process(s, &c)));
        }
        for pip in self.pipelines.iter().rev() {
            let post = pip.clone();
            let err = pip.clone();
            result = Box::new(
                result
                    .map(move |s| {
                        post.post(&s);
                        s
                    })
                    .map_err(move |e| {
                        err.error(&e);
                        e
                    }),
            )
        }
        Box::new(result.then(|s| match s {
            Ok(res) => ok::<Response<Body>, hyper::Error>(res.upstream_response),
            Err(_) => ok::<Response<Body>, hyper::Error>(Response::default()),
        }))
    }
}
