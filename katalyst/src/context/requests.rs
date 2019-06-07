use crate::prelude::*;
use crate::util::*;
use futures::{future::*, stream::Stream, Future};
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

impl Default for HttpRequest {
    fn default() -> Self {
        HttpRequest::Empty
    }
}

impl RequestContext {
    pub fn preload(&self) -> ModuleResult {
        let guard = self.clone();
        let req = ensure_fut!(guard.take_http_request());
        match req {
            HttpRequest::RawRequest(r) => {
                let (data, body) = (r.0, r.1);
                Box::new(body.concat2().then(move |r| match r {
                    Ok(body) => {
                        let res = Box::new((data, body.into_iter().collect()));
                        guard.set_http_request(HttpRequest::LoadedRequest(res))?;
                        Ok(())
                    }
                    Err(_) => Err(GatewayError::InternalServerError),
                }))
            }
            HttpRequest::RawResponse(r) => {
                let (data, body) = (r.0, r.1);
                Box::new(body.concat2().then(move |r| match r {
                    Ok(body) => {
                        let res = Box::new((data, body.into_iter().collect()));
                        guard.set_http_request(HttpRequest::LoadedResponse(res))?;
                        Ok(())
                    }
                    Err(_) => Err(GatewayError::InternalServerError),
                }))
            }
            _ => {
                ensure_fut!(guard.set_http_request(req));
                Box::new(ok(()))
            }
        }
    }

    pub fn parse(&self) -> ModuleResult {
        let guard = self.clone();
        Box::new(self.preload().and_then(move |_| {
            let hdr = guard.header("Content-Type").unwrap_or_default();
            let format = Format::content_type(Some(&hdr));
            let mut req = guard.take_http_request()?;
            if let HttpRequest::LoadedRequest(boxed) = req {
                let (data, body) = *boxed;
                let doc = match format.parse(&body) {
                    Ok(d) => d,
                    Err(_) => Document::Unit,
                };
                req = HttpRequest::ParsedRequest(Box::new((data, body, doc)));
            } else if let HttpRequest::LoadedResponse(boxed) = req {
                let (data, body) = *boxed;
                let doc = match format.parse(&body) {
                    Ok(d) => d,
                    Err(_) => Document::Unit,
                };
                req = HttpRequest::ParsedResponse(Box::new((data, body, doc)));
            }
            guard.set_http_request(req)?;
            Ok(())
        }))
    }

    pub fn get_http_request(&self) -> Result<Resource<HttpRequest>> {
        Ok(self.request.get())
    }

    pub fn take_http_request(&self) -> Result<HttpRequest> {
        Ok(self.request.take())
    }

    pub fn set_http_request(&self, inreq: HttpRequest) -> Result<HttpRequest> {
        Ok(self.request.set(inreq))
    }

    pub fn take_request(&self) -> Result<Request<Body>> {
        let res: HttpRequest = self.take_http_request()?;
        Ok(match res {
            HttpRequest::RawRequest(data) => Request::from_parts(data.0, data.1),
            HttpRequest::LoadedRequest(data) => Request::from_parts(data.0, Body::from(data.1)),
            HttpRequest::ParsedRequest(data) => Request::from_parts(data.0, Body::from(data.1)),
            _ => Request::default(),
        })
    }

    pub fn take_response(&self) -> Result<Response<Body>> {
        let res: HttpRequest = self.take_http_request()?;
        Ok(match res {
            HttpRequest::RawResponse(data) => Response::from_parts(data.0, data.1),
            HttpRequest::LoadedResponse(data) => Response::from_parts(data.0, Body::from(data.1)),
            HttpRequest::ParsedResponse(data) => Response::from_parts(data.0, Body::from(data.1)),
            _ => Response::default(),
        })
    }

    pub fn method(&self) -> http::Method {
        let req: &HttpRequest = &self.request.get();
        match req {
            HttpRequest::RawRequest(r) => r.0.method.clone(),
            HttpRequest::LoadedRequest(r) => r.0.method.clone(),
            HttpRequest::ParsedRequest(r) => r.0.method.clone(),
            _ => http::Method::GET,
        }
    }

    pub fn header(&self, key: &str) -> Option<String> {
        let req: &HttpRequest = &self.request.get();
        let prts = match req {
            HttpRequest::RawRequest(r) => &r.0,
            HttpRequest::LoadedRequest(r) => &r.0,
            HttpRequest::ParsedRequest(r) => &r.0,
            _ => return None,
        };
        if let Some(h) = prts.headers.get(key).map(|h| h.to_str().unwrap_or_default()) {
            Some(h.to_owned())
        } else {
            None
        }
    }

    pub fn set_response(&self, rsp: Response<Body>) -> Result<()> {
        self.set_http_request(HttpRequest::RawResponse(Box::new(rsp.into_parts())))?;
        Ok(())
    }

    pub fn is_request(&self) -> Result<bool> {
        let req: &HttpRequest = &self.request.get();
        Ok(match req {
            HttpRequest::RawRequest(_)
            | HttpRequest::LoadedRequest(_)
            | HttpRequest::ParsedRequest(_) => true,
            _ => false,
        })
    }

    pub fn is_response(&self) -> Result<bool> {
        let req: &HttpRequest = &self.request.get();
        Ok(match req {
            HttpRequest::RawResponse(_)
            | HttpRequest::LoadedResponse(_)
            | HttpRequest::ParsedResponse(_) => true,
            _ => false,
        })
    }
}

impl HttpRequest {
    pub fn new(req: Request<Body>) -> Self {
        HttpRequest::RawRequest(Box::new(req.into_parts()))
    }
}
