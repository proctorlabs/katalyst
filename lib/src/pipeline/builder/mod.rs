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

    fn process(&self, mut state: PipelineState, config: &Gateway) -> PipelineState {
        let mut path = String::new();
        let mut parts: Vec<Box<KatalystTemplatePlaceholder>> = vec![];
        {
            let route = state
                .matched_route
                .expect("Builder requires route to be matched already");
            path.push_str(&route.downstream.base_url);
            for part in route.downstream.path_parts.iter() {
                parts.push(part.duplicate()); //TODO: Gotta be a better way to do this...
            }
            state.matched_route = Some(route);
        }

        for part in parts.iter() {
            path.push_str(&part.get_value(&state, config));
        }

        let (mut parts, body) = state.upstream_request.into_parts();
        parts.uri = path.parse().unwrap();
        parts
            .headers
            .append("NewHeader", HeaderValue::from_str("Added").unwrap());
        let client_req = Request::from_parts(parts, body);

        state.upstream_request = Request::default();
        state.downstream_request = Some(client_req);

        state
    }
}
