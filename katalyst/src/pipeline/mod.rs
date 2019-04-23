mod auth;
mod dispatcher;
mod logger;
mod mapper;
mod matcher;

use crate::app::KatalystEngine;
use crate::prelude::*;
use futures::Future;
use hyper::{Body, Request};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub(crate) use mapper::HyperResult;

pub fn run(
    remote_addr: SocketAddr,
    request: Request<Body>,
    engine: Arc<KatalystEngine>,
) -> HyperResult {
    Box::new(
        ok!(Context::new(request, engine, remote_addr))
            .and_then(logger::log_request)
            .and_then(matcher::matcher)
            .and_then(dispatcher::run_plugins)
            .and_then(auth::authenticate)
            .and_then(auth::authorize)
            .and_then(dispatcher::run_handler)
            .map(logger::log_result)
            .map_err(logger::log_error)
            .then(mapper::map_result_to_hyper),
    )
}
