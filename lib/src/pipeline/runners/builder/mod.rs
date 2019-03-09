mod forwarding_headers;
mod hop_headers;

use crate::config::Gateway;
use crate::pipeline::*;
use hyper::Request;

pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn process(&self, mut state: PipelineState, config: &Gateway) -> PipelineResult {
        let state_ref = &state;
        let downstream = match &state_ref.context.matched_route {
            Some(route) => &route.downstream,
            None => {
                return self.fail(PipelineError::Failed {});
            }
        };

        let mut path = downstream.base_url.to_string();

        for part in downstream.path_parts.iter() {
            path.push_str(&part.get_value(&state, config));
        }

        let (mut parts, body) = state.upstream.request.unwrap().into_parts();
        debug!("Routing request to {}", path);
        parts.uri = path.parse().unwrap();
        parts = forwarding_headers::add_forwarding_headers(parts, state.remote_addr);
        parts = hop_headers::strip_hop_headers(parts);
        let client_req = Request::from_parts(parts, body);

        state.upstream.request = None;
        state.downstream.request = Some(client_req);

        self.ok(state)
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Builder {})
    }
}
