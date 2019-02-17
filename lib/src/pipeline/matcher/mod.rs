use super::*;
use crate::config::Gateway;
use hyper::{Body, StatusCode};

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway) -> PipelineState {
        for route in config.routes.iter() {
            if route.pattern.is_match(state.upstream_request.uri().path()) {
                state.matched_route = Some(route.clone());
                *state.upstream_response.body_mut() = Body::from("Matched!");
                return state;
            }
        }
        *state.upstream_response.status_mut() = StatusCode::NOT_FOUND;
        state
    }
}
