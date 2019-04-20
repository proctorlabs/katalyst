use crate::balancer;
use crate::config::parsers;
use crate::instance::KatalystState;
use crate::locator::{Locatable, Locator};
use crate::modules::Modules;
use crate::pipeline::PipelineRunner;
use crate::prelude::*;
use crate::service::EngineService;
use futures::future::Future;
use hyper::client::connect::dns::TokioThreadpoolGaiResolver;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::runtime::Runtime;

/// The Katalyst Engine
#[derive(Debug)]
pub struct KatalystEngine {
    state: RwLock<Arc<KatalystState>>,
    locator: Locator,
    pub rt: RwLock<Runtime>,
}

pub type HttpsClient = Client<HttpsConnector<HttpConnector<TokioThreadpoolGaiResolver>>, Body>;

impl Default for KatalystEngine {
    fn default() -> Self {
        let builder = Client::builder();
        let mut locator = Locator::default();
        let mut http_connector = HttpConnector::new_with_tokio_threadpool_resolver();
        http_connector.enforce_http(false);
        let mut tls = ClientConfig::new();
        tls.root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        locator.register(Compiler::default());
        locator.register::<HttpsClient>(builder.build(HttpsConnector::from((http_connector, tls))));
        locator.register(PipelineRunner::new());
        locator.register(balancer::all());
        locator.register(Modules::default());

        KatalystEngine {
            state: RwLock::default(),
            locator,
            rt: RwLock::new(Runtime::new().unwrap()),
        }
    }
}

impl KatalystEngine {
    /// Update the running configuration of the API Gateway.
    pub fn update_state(&self, new_state: KatalystState) -> Result<(), KatalystError> {
        let mut state = self.state.write()?;
        *state = Arc::new(new_state);
        Ok(())
    }

    pub fn locate<T: Locatable>(&self) -> Result<Arc<T>, KatalystError> {
        match self.locator.locate::<T>() {
            Some(t) => Ok(t),
            None => Err(KatalystError::FeatureUnavailable),
        }
    }

    /// Get a copy of the currently running API Gateway configuration.
    pub fn get_state(&self) -> Result<Arc<KatalystState>, KatalystError> {
        let state = self.state.read()?;
        Ok(state.clone())
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
}

#[derive(Default, Debug)]
pub struct Katalyst {
    engine: Arc<KatalystEngine>,
}

impl Katalyst {
    #[inline]
    fn engine(&self) -> Arc<KatalystEngine> {
        self.engine.clone()
    }

    /// Update the Katalyst instance with the configuration from the specified file.
    pub fn load(&self, config_file: &str) -> Result<(), KatalystError> {
        let config = parsers::parse_file(config_file)?;
        self.engine.update_state(config.build(self.engine())?)?;
        Ok(())
    }

    /// Run the Katalyst instance. This thread will block and run the async runtime.
    #[inline]
    pub fn run(&mut self) -> Result<(), KatalystError> {
        self.engine.run_service()?;
        self.engine.wait()?;
        Ok(())
    }

    /// This is a convenience method to start an instance of Katalyst from a configuration file.
    /// This will load the configuration from the specified file and run the gateway.
    pub fn start(config_file: &str) -> Result<Katalyst, KatalystError> {
        let mut app = Katalyst::default();
        app.load(config_file)?;
        app.run()?;
        Ok(app)
    }
}
