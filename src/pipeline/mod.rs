use crate::config::*;
use crate::matcher::Matcher;
use hyper::{Body, Request, Response};

#[derive(Debug)]
pub struct PipelineState {
    pub req: Request<Body>,
    pub rsp: Response<Body>,
    pub matched_route: Option<Route>,
}

impl PipelineState {
    fn new(request: Request<Body>) -> Self {
        PipelineState {
            req: request,
            rsp: Response::new(Body::empty()),
            matched_route: None,
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
        let pipelines: Vec<Box<Pipeline + Send>> = vec![Box::new(Matcher {})];
        PipelineRunner {
            pipelines: pipelines.into_boxed_slice(),
        }
    }

    pub fn run(&self, request: Request<Body>, config: &Gateway) -> PipelineState {
        let mut state = PipelineState::new(request);
        for pipeline in self.pipelines.iter() {
            if !(*pipeline).process(&mut state, config) {
                println!("Error in pipeline!");
                break;
            }
        }
        state
    }
}
