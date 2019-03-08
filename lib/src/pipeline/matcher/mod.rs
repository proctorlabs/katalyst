use super::*;
use crate::config::Gateway;
use futures::future::*;
use hyper::StatusCode;

pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway) -> PipelineResult {
        self.result({
            for route in config.routes.iter() {
                let method_match = match &route.methods {
                    Some(methods) => {
                        let up_method = state.upstream_request.method();
                        methods.contains(up_method)
                    }
                    None => true,
                };
                let path = state.upstream_request.uri().path();
                if method_match && route.pattern.is_match(path) {
                    let mut cap_map = HashMap::new();
                    let caps = route.pattern.captures(path).unwrap();
                    for name_option in route.pattern.capture_names() {
                        if name_option.is_some() {
                            let name = name_option.unwrap();
                            cap_map.insert(
                                name.to_string(),
                                caps.name(name).unwrap().as_str().to_string(),
                            );
                        }
                    }
                    state.matched_route = Box::new(Some(route.clone()));
                    state.captures = Some(cap_map);
                    debug!("Request has been matched to route!");
                    return Box::new(ok::<PipelineState, PipelineError>(state));
                }
            }
            state.return_status(StatusCode::NOT_FOUND);
            Err(PipelineError::Halted {})
        })
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Matcher {})
    }
}
