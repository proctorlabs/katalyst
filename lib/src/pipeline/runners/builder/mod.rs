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
        let downstream = match &state.context.matched_route {
            Some(route) => &route.downstream,
            None => {
                return Err(KatalystError::FeatureUnavailable);
            }
        };

        let balancer_lease = match config.hosts.get(&downstream.host) {
            Some(s) => s.servers.lease()?,
            None => {
                return Err(KatalystError::NotFound);
            }
        };
        let mut path = balancer_lease.to_string();
        state.context.balancer_lease = Some(balancer_lease);

        for part in downstream.path_parts.iter() {
            path.push_str(&part.get_value(&state, &config));
        }

        let (mut parts, body) = state.upstream.request.unwrap().into_parts();
        debug!("Routing request to {}", path);
        parts.uri = path.parse().unwrap();
        forwarding_headers::add_forwarding_headers(&mut parts.headers, state.remote_addr);
        hop_headers::strip_hop_headers(&mut parts.headers);
        let client_req = Request::from_parts(parts, body);

        state.upstream.request = None;
        state.downstream.request = Some(client_req);

        Ok(state)
    }

    fn process_response(&self, mut state: PipelineState) -> PipelineState {
        if let Some(r) = &mut state.upstream.response {
            hop_headers::strip_hop_headers(r.headers_mut());
        }
        state
    }
}
