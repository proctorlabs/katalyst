mod builder;
mod logger;
mod matcher;
mod sender;

use crate::config::{Gateway, Route};
use crate::service::BoxedFuture;
use builder::Builder;
use futures::future;
use hyper::{Body, Request, Response, StatusCode};
use logger::Logger;
use matcher::Matcher;
use sender::Sender;
use std::collections::HashMap;
use std::time::Instant;

pub struct PipelineState {
    pub upstream_request: Request<Body>,
    pub upstream_response: BoxedFuture,
    pub downstream_request: Option<Request<Body>>,
    pub downstream_response: Option<Response<Body>>,
    pub timestamps: HashMap<String, Instant>,
    pub matched_route: Box<Option<Route>>,
    pub finished: bool,
}

impl PipelineState {
    fn new(request: Request<Body>) -> Self {
        PipelineState {
            upstream_request: request,
            upstream_response: Box::new(future::ok(Response::default())), //Response::new(Body::empty()),
            downstream_request: None,
            downstream_response: None,
            matched_route: Box::new(None),
            timestamps: HashMap::new(),
            finished: false,
        }
    }

    fn return_status(&mut self, status: StatusCode) {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = status;
        self.finished = true;
        self.upstream_response = Box::new(future::ok(response));
    }
}

pub trait Pipeline {
    fn name(&self) -> &'static str;

    fn process(&self, state: PipelineState, _config: &Gateway) -> PipelineState {
        state
    }

    fn post(&self, _state: &PipelineState) {}

    fn error(&self, _state: &PipelineState) {}
}

pub struct PipelineRunner {
    pipelines: Box<[Box<Pipeline + Send>]>,
}

impl PipelineRunner {
    pub fn new() -> Self {
        //let mut state = PipelineState::new(request, config);
        let pipelines: Vec<Box<Pipeline + Send>> = vec![
            Box::new(Logger {}),
            Box::new(Matcher {}),
            Box::new(Builder {}),
            Box::new(Sender {}),
        ];
        PipelineRunner {
            pipelines: pipelines.into_boxed_slice(),
        }
    }

    pub fn run(&self, request: Request<Body>, config: &Gateway) -> PipelineState {
        let mut state = PipelineState::new(request);
        let mut last_run = 0;
        let mut error = false;
        for i in 0..self.pipelines.len() {
            last_run = i;
            let pipeline = &self.pipelines[i];
            debug!("Running pipeline {}", pipeline.name());
            state = pipeline.process(state, config);
            if state.finished {
                warn!("Error in pipeline {}!", pipeline.name());
                error = true;
                break;
            }
        }
        while last_run < self.pipelines.len() {
            let pipeline = &self.pipelines[last_run];
            debug!("Post actions for pipeline {}", pipeline.name());
            if error {
                pipeline.error(&state);
            } else {
                pipeline.post(&state);
            }
            if last_run > 0 {
                last_run -= 1;
            } else {
                break;
            }
        }
        state
    }
}
