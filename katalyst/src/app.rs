use crate::balancer;
use crate::config::parsers;
use crate::error::*;
use crate::instance::Instance;
use crate::modules::Modules;
use crate::pipeline::{run, HyperResult};
use crate::prelude::*;
use futures::future::Future;
use hyper::client::connect::dns::TokioThreadpoolGaiResolver;
use hyper::client::HttpConnector;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Server};
use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use std::fmt;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

pub type HttpsClient = Client<HttpsConnector<HttpConnector<TokioThreadpoolGaiResolver>>, Body>;

pub struct Katalyst {
    state: RwLock<Arc<Instance>>,
    client: Arc<HttpsClient>,
    balancers: Arc<balancer::BalancerDirectory>,
    compiler: Arc<Compiler>,
    modules: Modules,
    rt: RwLock<Runtime>,
}

impl std::fmt::Debug for Katalyst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Katalyst {{ instance: {:?} }}", self.state)
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
            state: RwLock::default(),
            client: Arc::new(builder.build(HttpsConnector::from((http_connector, tls)))),
            balancers: Arc::new(balancer::all()),
            compiler: Arc::new(Compiler::default()),
            modules: Modules::default(),
            rt: RwLock::new(Runtime::new().unwrap()),
        }
    }
}

pub trait ArcKatalystImpl {
    /// Update the Katalyst instance with the configuration from the specified file.
    fn load(&self, config_file: &str) -> Result<(), KatalystError>;

    /// Run the Katalyst instance. This thread will block and run the async runtime.
    fn run(&mut self) -> Result<(), KatalystError>;

    fn run_service(&mut self) -> Result<(), KatalystError>;
}

impl ArcKatalystImpl for Arc<Katalyst> {
    /// Update the Katalyst instance with the configuration from the specified file.
    fn load(&self, config_file: &str) -> Result<(), KatalystError> {
        let config = parsers::parse_file(config_file)?;
        self.update_state(config.build(self.clone())?)?;
        Ok(())
    }

    /// Run the Katalyst instance. This thread will block and run the async runtime.
    #[inline]
    fn run(&mut self) -> Result<(), KatalystError> {
        self.run_service()?;
        self.wait()?;
        Ok(())
    }

    fn run_service(&mut self) -> Result<(), KatalystError> {
        let engine = self.clone();
        let addr: SocketAddr = self.get_state()?.listener.interface.parse()?;
        let server = Server::bind(&addr)
            .serve(make_service_fn(move |conn: &AddrStream| {
                let engine = engine.clone();
                let remote_addr = conn.remote_addr();
                service_fn(move |req: Request<Body>| -> HyperResult {
                    run(remote_addr, req, engine.clone())
                })
            }))
            .map_err(|e| eprintln!("server error: {}", e));

        info!("Listening on http://{}", addr);
        self.spawn(server)
    }
}

impl Katalyst {
    /// Update the running configuration of the API Gateway.
    pub fn update_state(&self, new_state: Instance) -> Result<(), KatalystError> {
        let mut state = self.state.write()?;
        *state = Arc::new(new_state);
        Ok(())
    }

    pub fn get_balancers(&self) -> Arc<balancer::BalancerDirectory> {
        self.balancers.clone()
    }

    pub fn get_client(&self) -> Arc<HttpsClient> {
        self.client.clone()
    }

    pub fn get_compiler(&self) -> Arc<Compiler> {
        self.compiler.clone()
    }

    /// Get a copy of the currently running API Gateway configuration.
    pub fn get_state(&self) -> Result<Arc<Instance>, KatalystError> {
        let state = self.state.read()?;
        Ok(state.clone())
    }

    pub fn get_module(&self, name: &str, module_type: &str) -> Result<Arc<Module>, KatalystError> {
        self.modules.get(name, module_type)
    }

    pub fn spawn<F: Future<Item = (), Error = ()> + Send + 'static>(
        &self,
        fut: F,
    ) -> Result<(), KatalystError> {
        let mut rt = self.rt.write().unwrap();
        rt.spawn(fut);
        Ok(())
    }

    pub fn wait(&self) -> Result<(), KatalystError> {
        let mut rt = self.rt.write().unwrap();
        rt.block_on(futures::empty::<(), KatalystError>())?;
        Ok(())
    }

    /// This is a convenience method to start an instance of Katalyst from a configuration file.
    /// This will load the configuration from the specified file and run the gateway.
    pub fn start(config_file: &str) -> Result<Arc<Katalyst>, KatalystError> {
        let mut app = Arc::new(Katalyst::default());
        app.load(config_file)?;
        app.run()?;
        Ok(app)
    }
}
