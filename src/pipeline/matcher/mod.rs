use super::*;
use crate::config::Gateway;
use hyper::{Body, StatusCode};

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, state: &mut PipelineState, config: &Gateway) -> bool {
        for route in config.routes.iter() {
            if route.pattern.is_match(state.req.uri().path()) {
                println!(
                    "Message: {}",
                    match &route.message {
                        Some(s) => s.to_owned(),
                        None => String::from("No message"),
                    }
                );
                state.matched_route = Some(route.clone());
                *state.rsp.body_mut() = Body::from("Matched!");
                return true;
            }
        }
        *state.rsp.status_mut() = StatusCode::NOT_FOUND;
        false
    }
}
