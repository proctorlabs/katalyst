use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use futures::Future;
use http::request::Parts;
use hyper::{Body, Request, Response};
use unstructured::Document;

#[derive(Debug)]
pub enum HttpRequest {
    Empty,
    RawRequest(Box<(Parts, Body)>),
    LoadedRequest(Box<(Parts, Vec<u8>)>),
    ParsedRequest(Box<(Parts, Vec<u8>, Document)>),
    RawResponse(Box<(http::response::Parts, Body)>),
    LoadedResponse(Box<(http::response::Parts, Vec<u8>)>),
    ParsedResponse(Box<(http::response::Parts, Vec<u8>, Document)>),
}

impl Context {
    pub fn preload(mut self) -> ModuleResult {
        match &self.request {
            HttpRequest::RawRequest(_) => {
                let (data, body) = self.request.take_request().into_parts();
                Box::new(body.concat2().then(|r| match r {
                    Ok(body) => {
                        let res = Box::new((data, body.into_iter().collect()));
                        self.request = HttpRequest::LoadedRequest(res);
                        Ok(self)
                    }
                    Err(_) => Err(self.fail(GatewayError::InternalServerError)),
                }))
            }
            HttpRequest::RawResponse(_) => {
                let (data, body) = self.request.take_response().into_parts();
                Box::new(body.concat2().then(|r| match r {
                    Ok(body) => {
                        let res = Box::new((data, body.into_iter().collect()));
                        self.request = HttpRequest::LoadedResponse(res);
                        Ok(self)
                    }
                    Err(_) => Err(self.fail(GatewayError::InternalServerError)),
                }))
            }
            _ => Box::new(ok(self)),
        }
    }

    pub fn parse(self) -> ModuleResult {
        Box::new(self.preload().and_then(|mut slf| {
            let format = Format::content_type(slf.request.header("Content-Type"));
            match slf.request {
                HttpRequest::LoadedRequest(r) => {
                    let (data, body) = *r;
                    let doc = match format.parse(&body) {
                        Ok(d) => d,
                        Err(_) => Document::Unit,
                    };
                    slf.request = HttpRequest::ParsedRequest(Box::new((data, body, doc)));
                }
                HttpRequest::LoadedResponse(r) => {
                    let (data, body) = *r;
                    let doc = match format.parse(&body) {
                        Ok(d) => d,
                        Err(_) => Document::Unit,
                    };
                    slf.request = HttpRequest::ParsedResponse(Box::new((data, body, doc)));
                }
                _ => (),
            }
            Ok(slf)
        }))
    }
}

impl HttpRequest {
    pub fn new(req: Request<Body>) -> Self {
        HttpRequest::RawRequest(Box::new(req.into_parts()))
    }

    pub fn set_response(&mut self, rsp: Response<Body>) {
        std::mem::replace(
            self,
            HttpRequest::RawResponse(Box::new(rsp.into_parts())),
        );
    }

    pub fn is_request(&self) -> bool {
        match self {
            HttpRequest::RawRequest(_)
            | HttpRequest::LoadedRequest(_)
            | HttpRequest::ParsedRequest(_) => true,
            _ => false,
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            HttpRequest::RawResponse(_)
            | HttpRequest::LoadedResponse(_)
            | HttpRequest::ParsedResponse(_) => true,
            _ => false,
        }
    }

    fn parts(&self) -> Option<&Parts> {
        match self {
            HttpRequest::RawRequest(r) => Some(&r.0),
            HttpRequest::LoadedRequest(r) => Some(&r.0),
            HttpRequest::ParsedRequest(r) => Some(&r.0),
            _ => None,
        }
    }

    pub fn method(&self) -> &http::Method {
        if let Some(s) = self.parts() {
            &s.method
        } else {
            &http::Method::GET
        }
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        if let Some(s) = self.parts() {
            s.headers.get(key).map(|h| h.to_str().unwrap_or_default())
        } else {
            None
        }
    }

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, HttpRequest::Empty)
    }

    pub fn take_request(&mut self) -> Request<Body> {
        match std::mem::replace(self, HttpRequest::Empty) {
            HttpRequest::RawRequest(data) => Request::from_parts(data.0, data.1),
            HttpRequest::LoadedRequest(data) => {
                Request::from_parts(data.0, Body::from(data.1))
            }
            HttpRequest::ParsedRequest(data) => {
                Request::from_parts(data.0, Body::from(data.1))
            }
            _ => Request::default(),
        }
    }

    pub fn take_response(&mut self) -> Response<Body> {
        match std::mem::replace(self, HttpRequest::Empty) {
            HttpRequest::RawResponse(data) => Response::from_parts(data.0, data.1),
            HttpRequest::LoadedResponse(data) => {
                Response::from_parts(data.0, Body::from(data.1))
            }
            HttpRequest::ParsedResponse(data) => {
                Response::from_parts(data.0, Body::from(data.1))
            }
            _ => Response::default(),
        }
    }
}
