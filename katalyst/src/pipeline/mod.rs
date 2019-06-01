mod auth;
mod cache;
mod dispatcher;
mod logger;
mod mapper;
mod matcher;

use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use hyper::{Body, Request};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub(crate) use mapper::HyperResult;

pub(crate) fn run(
    remote_addr: SocketAddr,
    request: Request<Body>,
    engine: Arc<Katalyst>,
) -> HyperResult {
    Box::new(
        ok!(Context::new(request, engine, remote_addr))
            .and_then(logger::log_request)
            .and_then(matcher::matcher)
            .and_then(auth::authenticate)
            .and_then(auth::authorize)
            .and_then(cache::check_cache)
            .and_then(dispatcher::run_plugins)
            .and_then(dispatcher::run_handler)
            .and_then(cache::update_cache)
            .then(map_early_finish)
            .map(logger::log_result)
            .map_err(logger::log_error)
            .then(mapper::map_result_to_hyper),
    )
}

pub fn map_early_finish(res: ModuleResultSync) -> ModuleResult {
    match res {
        Err(ModuleError { error: GatewayError::Done, context }) => ok!(context),
        other => Box::new(futures::future::result(other)),
    }
}
