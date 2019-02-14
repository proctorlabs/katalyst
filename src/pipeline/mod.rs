mod logger;
mod matcher;

use crate::config::*;
use hyper::{Body, Request, Response};
use logger::Logger;
use matcher::Matcher;
use std::collections::HashMap;
use std::time::Instant;

pub struct PipelineState {
    pub req: Request<Body>,
    pub rsp: Response<Body>,
    pub timestamps: HashMap<String, Instant>,
    pub matched_route: Option<Route>,
}

impl PipelineState {
    fn new(request: Request<Body>) -> Self {
        PipelineState {
            req: request,
            rsp: Response::new(Body::empty()),
            matched_route: None,
            timestamps: HashMap::new(),
        }
    }
}

#[allow(unused_variables, unused_mut)]
pub trait Pipeline {
    fn name(&self) -> &'static str;

    fn process(&self, state: &mut PipelineState, config: &Gateway) -> bool {
        true
    }

    fn post(&self, state: &PipelineState) {}

    fn error(&self, state: &PipelineState) {}
}

pub struct PipelineRunner {
    pipelines: Box<[Box<Pipeline + Send>]>,
}

impl<'a> PipelineRunner {
    pub fn new() -> Self {
        //let mut state = PipelineState::new(request, config);
        let pipelines: Vec<Box<Pipeline + Send>> = vec![Box::new(Logger {}), Box::new(Matcher {})];
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
            if !(pipeline.process(&mut state, config)) {
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
