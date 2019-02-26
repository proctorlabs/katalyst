use super::*;
use crate::config::Gateway;
use crate::templates::KatalystTemplatePlaceholder;
use http::header::HeaderValue;
use hyper::Request;

pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway,) -> PipelineResult {
        if state.matched_route.is_none() {
            return self.fail(PipelineError::Failed{});
        }
        let mut path = String::new();
        let mut parts: Vec<&Box<KatalystTemplatePlaceholder>> = vec![];

        for route in state.matched_route.iter() {
            path.push_str(&route.downstream.base_url);
            parts = route.downstream.path_parts.iter().collect();
        }

        for part in parts.iter() {
            path.push_str(&part.get_value(&state, config));
        }

        let (mut parts, body) = state.upstream_request.into_parts();
        debug!("Routing request to {}", path);
        parts.uri = path.parse().unwrap();
        parts
            .headers
            .append("NewHeader", HeaderValue::from_str("Added").unwrap());
        let client_req = Request::from_parts(parts, body);

        state.upstream_request = Request::default();
        state.downstream_request = Some(client_req);

        self.ok(state)
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Builder {})
    }
}
