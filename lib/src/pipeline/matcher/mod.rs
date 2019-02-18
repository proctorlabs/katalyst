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
            let method_match = match &route.methods {
                Some(methods) => {
                    let up_method = state.upstream_request.method();
                    methods.contains(up_method)
                }
                None => true,
            };
            if method_match && route.pattern.is_match(state.upstream_request.uri().path()) {
                state.matched_route = Some(route.clone());
                return state;
            }
        }
        state.return_status(StatusCode::NOT_FOUND);
        state
    }
}
