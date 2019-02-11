use crate::config::Gateway;
use hyper::{Body, Request, Response, Server, StatusCode};

#[derive(Debug)]
pub struct PipelineState {
    pub config: Box<Gateway>,
    pub req: Request<Body>,
    pub rsp: Response<Body>,
}

impl PipelineState {
    pub fn new(request: Request<Body>, config: Gateway) -> Self {
        PipelineState {
            config: Box::new(config),
            req: request,
            rsp: Response::new(Body::empty()),
        }
    }
}

pub trait Pipeline {
    fn name(&self) -> &'static str;

    fn process(&self, state: &mut PipelineState) -> bool {
        true
    }

    fn post(&self, state: &PipelineState) {}

    fn error(&self, state: &PipelineState) {}
}
