use super::*;
use crate::config::Gateway;
use hyper::StatusCode;

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway) -> PipelineState {
        for route in config.routes.iter() {
            if route.pattern.is_match(state.upstream_request.uri().path()) {
                state.matched_route = Some(route.clone());
                return state;
            }
        }
        state.set_status(StatusCode::NOT_FOUND);
        state.failure = true;
        state
    }
}
