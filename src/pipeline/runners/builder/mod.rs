mod forwarding_headers;
mod hop_headers;
use crate::pipeline::*;

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

        let transformer = downstream.transformer(&state, balancer_lease.to_string())?;
        state.context.balancer_lease = Some(balancer_lease);

        let request = match state.upstream.request {
            Some(req) => req,
            None => return Err(KatalystError::FeatureUnavailable),
        };

        let mut client_req = transformer.transform(request)?;
        state.upstream.request = None;
        forwarding_headers::add_forwarding_headers(
            &mut client_req.headers_mut(),
            state.remote_addr,
        );
        hop_headers::strip_hop_headers(&mut client_req.headers_mut());
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
