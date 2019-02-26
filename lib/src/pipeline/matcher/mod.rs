use super::*;
use crate::config::Gateway;
use futures::future::*;
use hyper::StatusCode;

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway,) -> PipelineResult {
        for route in config.routes.iter() {
            let method_match = match &route.methods {
                Some(methods) => {
                    let up_method = state.upstream_request.method();
                    methods.contains(up_method)
                }
                None => true,
            };
            if method_match && route.pattern.is_match(state.upstream_request.uri().path()) {
                state.matched_route = Box::new(Some(route.clone()));
                return Box::new(ok::<PipelineState, PipelineError>(state));
            }
        }
        state.return_status(StatusCode::NOT_FOUND);
        self.fail(PipelineError::Halted{})
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Matcher {})
    }
}
