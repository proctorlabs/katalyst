use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::time::Instant;

use crate::config::Gateway;

type BoxedFuture = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn run_service(_config: Gateway) {
    let addr: SocketAddr = _config.listener.interface.parse().unwrap();
    let server = Server::bind(&addr)
        .serve(move || {
            let config = _config.clone();

            service_fn(move |req: Request<Body>| -> BoxedFuture {
                let start = Instant::now();
                let mut response = Response::new(Body::empty());

                loop {
                    for route in config.routes.iter() {
                        if route.pattern.is_match(req.uri().path()) {
                            *response.body_mut() = Body::from("Matched!");
                            break;
                        }
                    }
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    break;
                }
                println!(
                    "Processed in {}",
                    Instant::now().duration_since(start).subsec_micros()
                );
                Box::new(future::ok(response))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
