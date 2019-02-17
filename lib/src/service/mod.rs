use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;

use crate::config::Gateway;
use crate::pipeline::PipelineRunner;

type BoxedFuture = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn run_service(_config: Gateway) {
    let addr: SocketAddr = _config.listener.interface.parse().unwrap();
    let server = Server::bind(&addr)
        .serve(move || {
            let config = _config.clone();
            let pipeline = PipelineRunner::new();
            service_fn(move |req: Request<Body>| -> BoxedFuture {
                let result = pipeline.run(req, &config);
                Box::new(future::ok(result.upstream_response))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    info!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
