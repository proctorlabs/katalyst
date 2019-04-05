use crate::error::KatalystError;
use crate::pipeline::PipelineState;
use crate::templates::{StringTemplate, Templatizable};
use http::header::{HeaderName, HeaderValue};
use http::Method;
use hyper::Body;
use hyper::Request;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Downstream {
    pub host: String,
    pub path: StringTemplate,
    pub method: Option<Method>,
    pub query: Option<HashMap<String, StringTemplate>>,
    pub headers: Option<HashMap<String, StringTemplate>>,
    pub body: Option<StringTemplate>,
}

impl Downstream {
    pub fn build_request(
        &self,
        request: Request<Body>,
        state: &PipelineState,
        lease: &str,
    ) -> Result<Request<Body>, KatalystError> {
        let mut path = lease.to_string();
        path.push_str(&self.path.get_value(&state));

        if let Some(query) = &self.query {
            path.push_str("?");
            for (key, val) in query.iter() {
                path.push_str(&key);
                path.push_str("=");
                path.push_str(&val.get_value(&state));
                path.push_str("&");
            }
            path.truncate(path.len() - 1);
        }

        let (mut parts, mut body) = request.into_parts();
        parts.uri = path.parse().unwrap();

        if let Some(method) = &self.method {
            parts.method = method.clone();
        }

        if let Some(headers) = &self.headers {
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

        if let Some(body_str) = &self.body {
            body = hyper::Body::from(body_str.get_value(&state));
        }

        Ok(Request::from_parts(parts, body))
    }
}
