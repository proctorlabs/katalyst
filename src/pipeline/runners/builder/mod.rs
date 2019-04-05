mod forwarding_headers;
mod hop_headers;
use crate::pipeline::*;
use crate::templates::Templatizable;
use http::header::{HeaderName, HeaderValue};
use hyper::Request;
use std::str::FromStr;

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
        path.push_str(&downstream.path.get_value(&state));

        if let Some(query) = &downstream.query {
            path.push_str("?");
            for (key, val) in query.iter() {
                path.push_str(&key);
                path.push_str("=");
                path.push_str(&val.get_value(&state));
                path.push_str("&");
            }
            path.truncate(path.len() - 1);
        }

        let (mut parts, mut body) = state.upstream.request.unwrap().into_parts();
        state.upstream.request = None;
        debug!("Routing request to {}", path);

        parts.uri = path.parse().unwrap();
        forwarding_headers::add_forwarding_headers(&mut parts.headers, state.remote_addr);
        hop_headers::strip_hop_headers(&mut parts.headers);

        if let Some(method) = &downstream.method {
            parts.method = method.clone();
        }

        if let Some(headers) = &downstream.headers {
            for (key, val) in headers.iter() {
                while parts.headers.contains_key(key) {
                    parts.headers.remove(key);
                }
                let hdr_val = val.get_value(&state);
                if let (Ok(hdr), Ok(hdr_key)) =
                    (HeaderValue::from_str(&hdr_val), HeaderName::from_str(&key))
                {
                    parts.headers.append(hdr_key, hdr);
                }
            }
        }

        if let Some(body_str) = &downstream.body {
            body = hyper::Body::from(body_str.get_value(&state));
        }

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
