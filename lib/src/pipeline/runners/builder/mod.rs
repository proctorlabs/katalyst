mod forwarding_headers;
mod hop_headers;
use crate::pipeline::*;
use hyper::Request;

#[derive(Default)]
pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn prepare_request(&self, mut state: PipelineState) -> PipelineResult {
        let config = state.engine.get_state()?;
        let state_ref = &state;
        let downstream = match &state_ref.context.matched_route {
            Some(route) => &route.downstream,
            None => {
                return Err(KatalystError::Unavailable);
            }
        };

        let mut path = match config.hosts.get(&downstream.host) {
            Some(s) => s.servers[0].to_string(),
            None => {
                return Err(KatalystError::NotFound);
            }
        };

        for part in downstream.path_parts.iter() {
            path.push_str(&part.get_value(&state, &config));
        }

        let (mut parts, body) = state.upstream.request.unwrap().into_parts();
        debug!("Routing request to {}", path);
        parts.uri = path.parse().unwrap();
        parts = forwarding_headers::add_forwarding_headers(parts, state.remote_addr);
        parts = hop_headers::strip_hop_headers(parts);
        let client_req = Request::from_parts(parts, body);

        state.upstream.request = None;
        state.downstream.request = Some(client_req);

        Ok(state)
    }
}
