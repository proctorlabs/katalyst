mod auth;
mod cache;
mod dispatcher;
mod logger;
mod mapper;
mod matcher;

use crate::prelude::*;
use futures::Future;
use hyper::{Body, Request};
use std::{collections::HashMap, net::SocketAddr};

pub(crate) use mapper::HyperResult;

pub(crate) type PipelineResultSync = std::result::Result<RequestContext, ModuleError>;
pub(crate) type PipelineResult = Box<Future<Item = RequestContext, Error = ModuleError> + Send>;

macro_rules! pipe {
    ($ty:path) => {
        |ctx: RequestContext| {
            $ty(ctx.clone()).then(|res| match res {
                Ok(_) => Ok(ctx),
                Err(e) => Err(ModuleError { error: e, context: ctx }),
            })
        }
    };
}

pub(crate) fn run(
    remote_addr: SocketAddr,
    request: Request<Body>,
    engine: Katalyst,
) -> HyperResult {
    Box::new(
        ok(RequestContext::new(request, engine, remote_addr))
            .and_then(pipe!(logger::log_request))
            .and_then(pipe!(matcher::matcher))
            .and_then(pipe!(auth::authenticate))
            .and_then(pipe!(auth::authorize))
            .and_then(pipe!(cache::check_cache))
            .and_then(pipe!(dispatcher::run_plugins))
            .and_then(pipe!(dispatcher::run_handler))
            .and_then(pipe!(cache::update_cache))
            .then(map_early_finish)
            .map(logger::log_result)
            .map_err(logger::log_error)
            .then(mapper::map_result_to_hyper),
    )
}

pub(crate) fn map_early_finish(res: PipelineResultSync) -> PipelineResult {
    match res {
        Err(ModuleError { error: GatewayError::Done, context }) => {
            Box::new(ok::<RequestContext, ModuleError>(context))
        }
        other => Box::new(futures::future::result(other)),
    }
}
