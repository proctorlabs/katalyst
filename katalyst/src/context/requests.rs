use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use futures::Future;
use http::request::Parts;
use hyper::{Body, Request, Response};
use unstructured::Document;

#[derive(Debug)]
pub enum RequestContainer {
    Empty,
    RawRequest(Box<(Parts, Body)>),
    LoadedRequest(Box<(Parts, Vec<u8>)>),
    ParsedRequest(Box<(Parts, Vec<u8>, Document)>),
}

impl Context {
    pub fn preload(mut self) -> ModuleResult {
        match &self.request {
            RequestContainer::RawRequest(_) => {
                if let RequestContainer::RawRequest(r) = self.request.take() {
                    let (data, body) = *r;
                    Box::new(body.concat2().then(|r| match r {
                        Ok(body) => {
                            let res = Box::new((data, body.into_iter().collect()));
                            self.request = RequestContainer::LoadedRequest(res);
                            Ok(self)
                        }
                        Err(_) => Err(self.fail(GatewayError::InternalServerError)),
                    }))
                } else {
                    Box::new(ok(self))
                }
            }
            _ => Box::new(ok(self)),
        }
    }

    pub fn parse(self) -> ModuleResult {
        Box::new(self.preload().and_then(|mut slf| {
            let format = Format::content_type(slf.request.header("Content-Type"));
            if let RequestContainer::LoadedRequest(r) = slf.request {
                let (data, body) = *r;
                let doc = match format.parse(&body) {
                    Ok(d) => d,
                    Err(_) => Document::Unit,
                };
                slf.request = RequestContainer::ParsedRequest(Box::new((data, body, doc)));
            }
            Ok(slf)
        }))
    }
}

impl RequestContainer {
    pub fn new(req: Request<Body>) -> Self {
        RequestContainer::RawRequest(Box::new(req.into_parts()))
    }

    fn parts(&self) -> Option<&Parts> {
        match self {
            RequestContainer::RawRequest(r) => Some(&r.0),
            RequestContainer::LoadedRequest(r) => Some(&r.0),
            RequestContainer::ParsedRequest(r) => Some(&r.0),
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
        std::mem::replace(self, RequestContainer::Empty)
    }

    pub fn take_request(&mut self) -> Request<Body> {
        match std::mem::replace(self, RequestContainer::Empty) {
            RequestContainer::RawRequest(data) => Request::from_parts(data.0, data.1),
            RequestContainer::LoadedRequest(data) => {
                Request::from_parts(data.0, Body::from(data.1))
            }
            RequestContainer::ParsedRequest(data) => {
                Request::from_parts(data.0, Body::from(data.1))
            }
            _ => Request::default(),
        }
    }
}

#[derive(Debug)]
pub enum ResponseContainer {
    Empty,
    Raw { data: Box<Response<Body>> },
    Parsed,
}

impl ResponseContainer {
    pub fn new(req: Response<Body>) -> Self {
        ResponseContainer::Raw {
            data: Box::new(req),
        }
    }

    pub fn take(self) -> Response<Body> {
        match self {
            ResponseContainer::Empty => Response::default(),
            ResponseContainer::Raw { data } => *data,
            ResponseContainer::Parsed => unimplemented!(),
        }
    }
}
