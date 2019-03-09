mod runners;

use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::Route;
use futures::future::*;
use futures::Future;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

#[derive(Default)]
pub struct RequestResponse {
    pub request: Option<Request<Body>>,
    pub response: Option<Response<Body>>,
}

#[derive(Default)]
pub struct RequestContext {
    pub matched_route: Option<Arc<Route>>,
    pub captures: Option<HashMap<String, String>>,
    pub timestamps: HashMap<String, Instant>,
}

pub struct PipelineState {
    pub upstream: RequestResponse,
    pub downstream: RequestResponse,
    pub context: RequestContext,
    pub remote_addr: SocketAddr,
    pub engine: Arc<KatalystEngine>,
}

impl PipelineState {
    fn new(request: Request<Body>, engine: Arc<KatalystEngine>, remote: SocketAddr) -> Self {
        let mut state = PipelineState {
            upstream: RequestResponse::default(),
            downstream: RequestResponse::default(),
            context: RequestContext::default(),
            engine: engine,
            remote_addr: remote,
        };
        state.upstream.request = Some(request);
        state
    }
}

pub type HyperResult = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub type AsyncPipelineResult = Box<Future<Item = PipelineState, Error = KatalystError> + Send>;
pub type PipelineResult = Result<PipelineState, KatalystError>;

pub trait Pipeline: Send + Sync {
    fn name(&self) -> &'static str;

    fn process(&self, state: PipelineState) -> AsyncPipelineResult {
        Box::new(result(self.process_result(state)))
    }

    fn process_result(&self, _state: PipelineState) -> PipelineResult {
        Err(KatalystError::Unavailable)
    }

    fn post(&self, _state: &PipelineState) {}

    fn error(&self, _state: &KatalystError) {}

    fn arc() -> Arc<Pipeline>
    where
        Self: 'static + Sized + Default,
    {
        Arc::new(Self::default())
    }
}

pub struct PipelineRunner {
    pipelines: Arc<[Arc<Pipeline>]>,
}

impl PipelineRunner {
    pub fn new() -> Self {
        PipelineRunner {
            pipelines: runners::all(),
        }
    }

    pub fn run(
        &self,
        remote_addr: SocketAddr,
        request: Request<Body>,
        engine: Arc<KatalystEngine>,
    ) -> HyperResult {
        let mut result: AsyncPipelineResult = Box::new(lazy(move || {
            ok::<PipelineState, KatalystError>(PipelineState::new(request, engine, remote_addr))
        }));
        for pip in self.pipelines.iter() {
            result = Box::new(result.and_then({
                let r = pip.clone();
                move |s| r.process(s)
            }));
        }
        for pip in self.pipelines.iter().rev() {
            result = Box::new(
                result
                    .map({
                        let r = pip.clone();
                        move |s| {
                            r.post(&s);
                            s
                        }
                    })
                    .map_err({
                        let r = pip.clone();
                        move |e| {
                            r.error(&e);
                            e
                        }
                    }),
            )
        }
        Box::new(result.then(|s| match s {
            Ok(res) => ok::<Response<Body>, hyper::Error>(res.upstream.response.unwrap()),
            Err(e) => ok::<Response<Body>, hyper::Error>({
                let mut resp = Response::default();
                *resp.status_mut() = e.status_code();
                resp
            }),
        }))
    }
}
