mod runners;

use crate::app::HttpsClient;
use crate::config::{Gateway, Route};
use crate::error::KatalystError;
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
    pub matched_route: Option<Route>,
    pub captures: Option<HashMap<String, String>>,
    pub timestamps: HashMap<String, Instant>,
}

pub struct PipelineState {
    pub upstream: RequestResponse,
    pub downstream: RequestResponse,
    pub context: RequestContext,
    pub client: Arc<HttpsClient>,
    pub remote_addr: SocketAddr,
}

impl PipelineState {
    fn new(request: Request<Body>, client: Arc<HttpsClient>, remote: SocketAddr) -> Self {
        let mut state = PipelineState {
            upstream: RequestResponse::default(),
            downstream: RequestResponse::default(),
            context: RequestContext::default(),
            client: client,
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

    fn process(&self, state: PipelineState, config: &Gateway) -> AsyncPipelineResult {
        Box::new(result(self.process_result(state, config)))
    }

    fn process_result(&self, _state: PipelineState, _config: &Gateway) -> PipelineResult {
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
    client: Arc<HttpsClient>,
}

impl PipelineRunner {
    pub fn new(client: Arc<HttpsClient>) -> Self {
        PipelineRunner {
            pipelines: runners::all(),
            client: client,
        }
    }

    pub fn run(
        &self,
        remote_addr: SocketAddr,
        request: Request<Body>,
        inc_config: &Gateway,
    ) -> HyperResult {
        let config = Arc::new(inc_config.clone());
        let client = self.client.clone();
        let mut result: AsyncPipelineResult = Box::new(lazy(move || {
            ok::<PipelineState, KatalystError>(PipelineState::new(request, client, remote_addr))
        }));
        for pip in self.pipelines.iter() {
            let c = config.clone();
            let runner = pip.clone();
            result = Box::new(result.and_then(move |s| runner.process(s, &c)));
        }
        for pip in self.pipelines.iter().rev() {
            let post = pip.clone();
            let err = pip.clone();
            result = Box::new(
                result
                    .map(move |s| {
                        post.post(&s);
                        s
                    })
                    .map_err(move |e| {
                        err.error(&e);
                        e
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
