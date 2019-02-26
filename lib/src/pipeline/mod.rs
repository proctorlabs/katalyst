mod builder;
mod logger;
mod matcher;
mod sender;

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
use std::time::Instant;

pub struct PipelineState {
    pub upstream_request: Request<Body>,
    pub upstream_response: Response<Body>,
    pub downstream_request: Option<Request<Body>>,
    pub downstream_response: Option<Response<Body>>,
    pub timestamps: HashMap<String, Instant>,
    pub matched_route: Box<Option<Route>>,
    pub finished: bool,
    pub hyper_error: Option<hyper::Error>,
}

impl PipelineState {
    fn new(request: Request<Body>) -> Self {
        PipelineState {
            upstream_request: request,
            upstream_response: Response::default(), //Response::new(Body::empty()),
            downstream_request: None,
            downstream_response: None,
            matched_route: Box::new(None),
            timestamps: HashMap::new(),
            finished: false,
            hyper_error: None,
        }
    }

    fn return_status(&mut self, status: StatusCode) {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = status;
        self.finished = true;
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

    fn error(&self, _state: &PipelineState) {}

    fn make(&self) -> Box<Pipeline + Send + Sync>;

    fn ok(&self, state: PipelineState) -> PipelineResult {
        Box::new(ok::<PipelineState, PipelineError>(state))
    }

    fn fail(&self, error: PipelineError) -> PipelineResult {
        Box::new(err::<PipelineState, PipelineError>(error))
    }
}

pub struct PipelineRunner {
    pipelines: Vec<Box<Pipeline + Send + Sync>>,
}

impl PipelineRunner {
    pub fn new() -> Self {
        let pipelines: Vec<Box<Pipeline + Send + Sync>> = vec![
            Box::new(Logger {}),
            Box::new(Matcher {}),
            Box::new(Builder {}),
            Box::new(Sender {}),
        ];
        PipelineRunner {
            pipelines: pipelines,
        }
    }

    pub fn run(&self, request: Request<Body>, config: &Gateway) -> HyperResult {
        let mut result: Box<Future<Item = PipelineState, Error = PipelineError> + Send> =
            Box::new(lazy(|| {
                ok::<PipelineState, PipelineError>(PipelineState::new(request))
            }));
        let mut pipelines: Vec<Box<Pipeline + Send + Sync>> = vec![];
        for pip in self.pipelines.iter() {
            pipelines.push(pip.make());
        }
        while !pipelines.is_empty() {
            let runner = pipelines.remove(0);
            let c = config.clone();
            result = Box::new(result.and_then(move |s| runner.process(s, &c)));
        }
        Box::new(result.then(|s| match s {
            Ok(res) => ok::<Response<Body>, hyper::Error>(res.upstream_response),
            Err(_) => ok::<Response<Body>, hyper::Error>(Response::new(Body::empty())),
        }))
    }
}
