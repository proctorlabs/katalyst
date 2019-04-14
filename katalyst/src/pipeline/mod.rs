mod runners;

use crate::app::KatalystEngine;
use crate::prelude::*;
use futures::future::*;
use futures::Future;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub(crate) type HyperResult = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub type AsyncPipelineResult = Box<Future<Item = Context, Error = RequestFailure> + Send>;
pub type PipelineResult = Result<Context, RequestFailure>;

pub trait Pipeline: Send + Sync {
    fn name(&self) -> &'static str;

    fn prepare_request_future(&self, ctx: Context) -> AsyncPipelineResult {
        Box::new(result(self.prepare_request(ctx)))
    }

    fn prepare_request(&self, ctx: Context) -> PipelineResult {
        Ok(ctx)
    }

    fn process_response(&self, ctx: Context) -> Context {
        ctx
    }

    fn process_error(&self, err: RequestFailure) -> RequestFailure {
        err
    }
}

pub(crate) struct PipelineRunner {
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
            ok::<Context, RequestFailure>(Context::new(request, engine, remote_addr))
        }));
        for pip in self.pipelines.iter() {
            result = Box::new(result.and_then({
                let r = pip.clone();
                move |s| r.prepare_request_future(s)
            }));
        }
        for pip in self.pipelines.iter().rev() {
            result = Box::new(
                result
                    .map({
                        let r = pip.clone();
                        move |s| r.process_response(s)
                    })
                    .map_err({
                        let r = pip.clone();
                        move |e| r.process_error(e)
                    }),
            )
        }
        Box::new(result.then(|s| match s {
            Ok(res) => {
                ok::<Response<Body>, hyper::Error>(res.upstream.response.unwrap_or_default())
            }
            Err(e) => ok::<Response<Body>, hyper::Error>({
                let mut resp = Response::default();
                *resp.status_mut() = e.status_code();
                resp
            }),
        }))
    }
}
