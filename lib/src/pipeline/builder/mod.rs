use super::*;
use crate::config::Gateway;
use http::header::HeaderValue;
use hyper::Request;

pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineState {
        {
            let route = state
                .matched_route
                .expect("Builder requires route to be matched already");
            let mut path = route.downstream.base_url.to_owned();
            path.push_str(&route.downstream.path);

            let (mut parts, body) = state.upstream_request.into_parts();
            parts.uri = path.parse().unwrap();
            parts
                .headers
                .append("NewHeader", HeaderValue::from_str("Added").unwrap());
            let client_req = Request::from_parts(parts, body);

            state.upstream_request = Request::default();
            state.downstream_request = Some(client_req);
            state.matched_route = Some(route);
        }
        state
    }
}
