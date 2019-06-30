mod pipeline;

use crate::{instance::Interface, prelude::*};
use futures::{stream::Stream, Future};
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request,
};
use pipeline::{run, HyperResult};
use rustls::internal::pemfile;
use std::{fs, io, net::SocketAddr, sync::Arc};
use tokio_rustls::TlsAcceptor;

pub(crate) enum Server {
    Http(HttpServer),
    Https(HttpsServer),
}

pub(crate) trait Service {
    fn spawn(&self, _: Katalyst) -> Result<()>;
}

impl Server {
    pub fn new(iface: &Interface) -> Result<Server> {
        Ok(match iface {
            Interface::Http { addr } => Server::Http(HttpServer { addr: *addr }),
            Interface::Https { addr, cert, key } => Server::Https(HttpsServer {
                http: HttpServer { addr: *addr },
                cert: cert.to_owned(),
                key: key.to_owned(),
            }),
        })
    }
}

impl Service for Server {
    fn spawn(&self, katalyst: Katalyst) -> Result<()> {
        match self {
            Server::Http(s) => s.spawn(katalyst),
            Server::Https(s) => s.spawn(katalyst),
        }
    }
}

pub(crate) struct HttpServer {
    addr: SocketAddr,
}

impl Service for HttpServer {
    fn spawn(&self, instance: Katalyst) -> Result<()> {
        let engine = instance.clone();
        let server = hyper::Server::bind(&self.addr)
            .serve(make_service_fn(move |conn: &AddrStream| {
                let engine = engine.clone();
                let remote_addr = conn.remote_addr();
                service_fn(move |req: Request<Body>| -> HyperResult {
                    run(remote_addr, req, engine.clone())
                })
            }))
            .map_err(|e| error!("server error: {}", e));

        info!("Listening on http://{}", self.addr);
        instance.spawn(server)?;
        Ok(())
    }
}

pub(crate) struct HttpsServer {
    http: HttpServer,
    cert: String,
    key: String,
}

impl Service for HttpsServer {
    fn spawn(&self, instance: Katalyst) -> Result<()> {
        let engine = instance.clone();
        let tls_cfg = {
            let certs = self.load_certs(&self.cert)?;
            let key = self.load_private_key(&self.key)?;
            let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            cfg.set_single_cert(certs, key).unwrap();
            Arc::new(cfg)
        };

        let tcp = tokio_tcp::TcpListener::bind(&self.http.addr)?;
        let tls_acceptor = TlsAcceptor::from(tls_cfg);
        let tls = tcp
            .incoming()
            .and_then(move |s| tls_acceptor.accept(s))
            .then(|r| match r {
                Ok(x) => Ok::<_, io::Error>(Some(x)),
                Err(_e) => Ok(None),
            })
            .filter_map(|x| x);
        let server = hyper::Server::builder(tls)
            .serve(make_service_fn(
                move |conn: &tokio_rustls::TlsStream<
                    tokio_tcp::TcpStream,
                    rustls::ServerSession,
                >| {
                    let remote_addr = conn.get_ref().0.peer_addr().unwrap();
                    let engine = engine.clone();
                    service_fn(move |req: Request<Body>| -> HyperResult {
                        run(remote_addr, req, engine.clone())
                    })
                },
            ))
            .map_err(|e| error!("server error: {}", e));

        info!("Listening on https://{}", self.http.addr);
        instance.spawn(server)?;
        Ok(())
    }
}

impl HttpsServer {
    fn load_certs(&self, filename: &str) -> Result<Vec<rustls::Certificate>> {
        let certfile = fs::File::open(filename)?;
        let mut reader = io::BufReader::new(certfile);
        pemfile::certs(&mut reader).map_err(|_| err!(Critical, "Failed to load certificate"))
    }

    fn load_private_key(&self, filename: &str) -> Result<rustls::PrivateKey> {
        let keyfile = fs::File::open(filename)?;
        let mut reader = io::BufReader::new(keyfile);
        let mut keys = pemfile::rsa_private_keys(&mut reader)
            .map_err(|_| err!(Critical, "Failed to load private key"))?;
        if keys.len() != 1 {
            return Err(err!(Critical, "Expected a single private key"));
        }
        Ok(keys.pop().unwrap())
    }
}
