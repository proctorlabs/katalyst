use super::*;
use crate::config::Gateway;
use hyper::Method;
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
                    let mut up_string = state.upstream_request.method().to_string();
                    up_string.make_ascii_lowercase();
                    let up_method = Method::from_bytes(up_string.as_bytes()).unwrap();
                    methods.contains(&up_method)
                }
                None => true,
            };
            println!("Matched: {}", method_match);
            if method_match && route.pattern.is_match(state.upstream_request.uri().path()) {
                state.matched_route = Some(route.clone());
                return state;
            }
        }
        state.set_status(StatusCode::NOT_FOUND);
        state.failure = true;
        state
    }
}
