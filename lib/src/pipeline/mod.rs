mod builder;
mod logger;
mod matcher;
mod sender;

use crate::config::*;
use builder::Builder;
use hyper::{Body, Request, Response};
use logger::Logger;
use matcher::Matcher;
use sender::Sender;
use std::collections::HashMap;
use std::time::Instant;

pub struct PipelineState {
    pub upstream_request: Request<Body>,
    pub upstream_response: Response<Body>,
    pub downstream_request: Option<Request<Body>>,
    pub downstream_response: Option<Response<Body>>,
    pub timestamps: HashMap<String, Instant>,
    pub matched_route: Option<Route>,
    pub failure: bool,
}

impl PipelineState {
    fn new(request: Request<Body>) -> Self {
        PipelineState {
            upstream_request: request,
            upstream_response: Response::new(Body::empty()),
            downstream_request: None,
            downstream_response: None,
            matched_route: None,
            timestamps: HashMap::new(),
            failure: false,
        }
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

impl<'a> PipelineRunner {
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
        let mut end = 0;
        let mut error = false;
        for i in 0..self.pipelines.len() {
            end = i;
            let pipeline = &self.pipelines[i];
            println!("Running pipeline {}", pipeline.name());
            state = pipeline.process(state, config);
            if state.failure {
                println!("Error in pipeline {}!", pipeline.name());
                error = true;
                break;
            }
        }
        while end < self.pipelines.len() {
            let pipeline = &self.pipelines[end];
            println!("Post actions for pipeline {}", pipeline.name());
            if error == true {
                pipeline.error(&state);
            } else {
                pipeline.post(&state);
            }
            if end > 0 {
                end = end - 1;
            } else {
                end = self.pipelines.len();
            }
        }
        state
    }
}
