use crate::balancer;
use crate::config::parsers;
use crate::error::*;
use crate::instance::Instance;
use crate::instance::Interface;
use crate::modules::ModuleRegistry;
use crate::pipeline::{run, HyperResult};
use crate::prelude::*;
use hyper::client::connect::dns::TokioThreadpoolGaiResolver;
use hyper::client::HttpConnector;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Server};
use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use signal_hook::iterator::Signals;
use signal_hook::{SIGINT, SIGQUIT, SIGTERM};
use std::fmt;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

use futures::stream::Stream;
use hyper::rt::Future;
use rustls::internal::pemfile;
use std::{fs, io};
use tokio_rustls::TlsAcceptor;

pub type HttpsClient = Client<HttpsConnector<HttpConnector<TokioThreadpoolGaiResolver>>, Body>;

/// This is the core structure for the API Gateway.
pub struct Katalyst {
    instance: RwLock<Arc<Instance>>,
    client: Arc<HttpsClient>,
    balancers: Arc<balancer::BalancerDirectory>,
    compiler: Arc<Compiler>,
    modules: ModuleRegistry,
    rt: RwLock<Runtime>,
}

impl std::fmt::Debug for Katalyst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Katalyst {{ instance: {:?} }}", self.instance)
    }
}

impl Default for Katalyst {
    fn default() -> Self {
        let builder = Client::builder();
        let mut http_connector = HttpConnector::new_with_tokio_threadpool_resolver();
        http_connector.enforce_http(false);
        let mut tls = ClientConfig::new();
        tls.root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        Katalyst {
            instance: RwLock::default(),
            client: Arc::new(builder.build(HttpsConnector::from((http_connector, tls)))),
            balancers: Arc::new(balancer::all()),
            compiler: Arc::new(Compiler::default()),
            modules: ModuleRegistry::default(),
            rt: RwLock::new(Runtime::new().unwrap()),
        }
    }
}

pub trait ArcKatalystImpl {
    /// Update the Katalyst instance with the configuration from the specified file.
    fn load(&self, config_file: &str) -> Result<()>;

    /// Run the Katalyst instance. This thread will block and run the async runtime.
    fn run(&mut self) -> Result<()>;

    fn run_service(&mut self) -> Result<()>;
}

impl ArcKatalystImpl for Arc<Katalyst> {
    /// Update the Katalyst instance with the configuration from the specified file.
    fn load(&self, config_file: &str) -> Result<()> {
        let config = parsers::parse_file(config_file)?;
        self.update_instance(config.build(self.clone())?)?;
        Ok(())
    }

    /// Run the Katalyst instance. This thread will block and run the async runtime.
    #[inline]
    fn run(&mut self) -> Result<()> {
        self.run_service()?;
        self.wait()?;
        Ok(())
    }

    fn run_service(&mut self) -> Result<()> {
        let instance = self.get_instance()?.clone();
        for interface in instance.service.interfaces.iter() {
            match interface {
                Interface::Http { addr } => {
                    let engine = self.clone();
                    let server = Server::bind(&addr)
                        .serve(make_service_fn(move |conn: &AddrStream| {
                            let engine = engine.clone();
                            let remote_addr = conn.remote_addr();
                            service_fn(move |req: Request<Body>| -> HyperResult {
                                run(remote_addr, req, engine.clone())
                            })
                        }))
                        .map_err(|e| error!("server error: {}", e));

                    info!("Listening on http://{}", addr);
                    self.spawn(server)?;
                }
                Interface::Https { addr, cert, key } => {
                    let addr = addr.clone();
                    let engine = self.clone();
                    let tls_cfg = {
                        let certs = load_certs(&cert)?;
                        let key = load_private_key(&key)?;
                        let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
                        cfg.set_single_cert(certs, key).unwrap();
                        Arc::new(cfg)
                    };

                    let tcp = tokio_tcp::TcpListener::bind(&addr)?;
                    let tls_acceptor = TlsAcceptor::from(tls_cfg);
                    let tls = tcp
                        .incoming()
                        .and_then(move |s| tls_acceptor.accept(s))
                        .then(|r| match r {
                            Ok(x) => Ok::<_, io::Error>(Some(x)),
                            Err(_e) => Ok(None),
                        })
                        .filter_map(|x| x);
                    let server = Server::builder(tls)
                        .serve(
                            //make_service_fn(move |conn: &AddrStream| {
                            //let engine = engine.clone();
                            //let remote_addr = conn.remote_addr();
                            move || {
                                let addr = addr.clone();
                                let engine = engine.clone();
                                service_fn(move |req: Request<Body>| -> HyperResult {
                                    run(addr.clone(), req, engine.clone())
                                })
                            },
                        )
                        .map_err(|e| error!("server error: {}", e));

                    info!("Listening on https://{}", addr);
                    self.spawn(server)?;
                }
            }
        }
        Ok(())
    }
}

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    pemfile::certs(&mut reader).map_err(|_| error("failed to load certificate".into()))
}

fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    let keys = pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;
    println!("{}", keys.len());
    if keys.len() != 1 {
        return Err(error("expected a single private key".into()));
    }
    Ok(keys[0].clone())
}

impl Katalyst {
    /// Update the running configuration of the API Gateway.
    pub fn update_instance(&self, new_instance: Instance) -> Result<()> {
        let mut instance = self.instance.write()?;
        *instance = Arc::new(new_instance);
        Ok(())
    }

    /// Get a copy of the currently running API Gateway configuration.
    pub fn get_instance(&self) -> Result<Arc<Instance>> {
        let instance = self.instance.read()?;
        Ok(instance.clone())
    }

    #[inline]
    pub(crate) fn get_balancers(&self) -> Arc<balancer::BalancerDirectory> {
        self.balancers.clone()
    }

    #[inline]
    pub(crate) fn get_client(&self) -> Arc<HttpsClient> {
        self.client.clone()
    }

    #[inline]
    pub(crate) fn get_compiler(&self) -> Arc<Compiler> {
        self.compiler.clone()
    }

    #[inline]
    pub(crate) fn get_module(&self, name: &str) -> Result<Arc<ModuleProvider>> {
        self.modules.get(name)
    }

    pub fn spawn<F: Future<Item = (), Error = ()> + Send + 'static>(&self, fut: F) -> Result<()> {
        let mut rt = self.rt.write().unwrap();
        rt.spawn(fut);
        Ok(())
    }

    pub fn wait(&self) -> Result<()> {
        let signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT])?;
        for sig in signals.forever() {
            match sig {
                SIGINT | SIGTERM | SIGQUIT => break,
                _ => (),
            };
        }
        info!("Signal received, shutting down...");
        Ok(())
    }

    /// This is a convenience method to start an instance of Katalyst from a configuration file.
    /// This will load the configuration from the specified file and run the gateway.
    pub fn start(config_file: &str) -> Result<Arc<Katalyst>> {
        let mut app = Arc::new(Katalyst::default());
        app.load(config_file)?;
        app.run()?;
        Ok(app)
    }
}
