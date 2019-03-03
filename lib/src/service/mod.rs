use hyper::rt::Future;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Request, Server};
use std::net::SocketAddr;

use crate::app::*;
use crate::error::*;
use crate::pipeline::HyperResult;
use crate::pipeline::PipelineRunner;
use std::sync::Arc;

pub fn run_service(engine: Arc<KatalystEngine>) -> Result<(), KatalystError> {
    let addr: SocketAddr = engine.get_state()?.listener.interface.parse()?;
    let server = Server::bind(&addr)
        .serve(make_service_fn(move |conn: &AddrStream| {
            let engine = engine.clone();
            let pipeline = PipelineRunner::new();
            let _remote_addr = conn.remote_addr();
            service_fn(move |req: Request<Body>| -> HyperResult {
                let config = engine.get_state().unwrap();
                pipeline.run(req, &config)
            })
        }))
        .map_err(|e| eprintln!("server error: {}", e));

    info!("Listening on http://{}", addr);
    hyper::rt::run(server);
    Ok(())
}
