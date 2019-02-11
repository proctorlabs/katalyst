use crate::pipeline::*;
use hyper::{Body, StatusCode};

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, state: &mut PipelineState) -> bool {
        loop {
            for route in state.config.routes.iter() {
                if route.pattern.is_match(state.req.uri().path()) {
                    *state.rsp.body_mut() = Body::from("Matched!");
                    break;
                }
            }
            *state.rsp.status_mut() = StatusCode::NOT_FOUND;
            break;
        }
        true
    }
}
