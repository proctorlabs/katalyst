use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Server};
use std::net::SocketAddr;

use crate::config::Gateway;
use crate::pipeline::PipelineRunner;
use crate::pipeline::HyperResult;

pub fn run_service(_config: Gateway) {
    let addr: SocketAddr = _config.listener.interface.parse().unwrap();
    let server = Server::bind(&addr)
        .serve(move || {
            let config = _config.clone();
            let pipeline = PipelineRunner::new();
            service_fn(move |req: Request<Body>| -> HyperResult { pipeline.run(req, &config) })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    info!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
