use crate::prelude::*;
use http::header::{HeaderName, HeaderValue};
use http::Method;
use hyper::Body;
use hyper::Request;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct DownstreamTransformer {
    pub uri: String,
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl DownstreamTransformer {
    pub fn transform(self, req: Request<Body>) -> Result<Request<Body>, GatewayError> {
        let (mut parts, mut body) = req.into_parts();
        parts.uri = self.uri.parse()?;

        if let Some(method) = self.method {
            parts.method = method;
        }

        if let Some(body_str) = self.body {
            while parts.headers.contains_key("Content-Length") {
                parts.headers.remove("Content-Length");
            }
            body = hyper::Body::from(body_str);
        }

        if let Some(headers) = self.headers {
            for (key_str, val_str) in headers.iter() {
                if let (Ok(key), Ok(val)) = (
                    HeaderName::from_str(&key_str),
                    HeaderValue::from_str(val_str),
                ) {
                    while parts.headers.contains_key(key_str) {
                        parts.headers.remove(key_str);
                    }
                    parts.headers.append(key, val);
                }
            }
        }

        Ok(Request::from_parts(parts, body))
    }
}
