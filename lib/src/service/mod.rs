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

pub trait EngineService {
    fn run_service(&mut self) -> Result<(), KatalystError>;
}

impl EngineService for Arc<KatalystEngine> {
    fn run_service(&mut self) -> Result<(), KatalystError> {
        let engine = self.clone();
        let addr: SocketAddr = self.get_state()?.listener.interface.parse()?;
        let server = Server::bind(&addr)
            .serve(make_service_fn(move |conn: &AddrStream| {
                let engine = engine.clone();
                let pipeline = engine.locate::<PipelineRunner>().unwrap();
                let remote_addr = conn.remote_addr();
                service_fn(move |req: Request<Body>| -> HyperResult {
                    pipeline.run(remote_addr, req, engine.clone())
                })
            }))
            .map_err(|e| eprintln!("server error: {}", e));

        info!("Listening on http://{}", addr);
        self.spawn(server)
    }
}
