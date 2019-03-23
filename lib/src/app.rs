use crate::authentication;
use crate::balancer;
use crate::config::parsers;
use crate::error::*;
use crate::locator::{Locatable, Locator};
use crate::pipeline::PipelineRunner;
use crate::service::EngineService;
use crate::state::KatalystState;
use crate::templates::Providers;
use futures::future::Future;
use hyper::client::connect::dns::TokioThreadpoolGaiResolver;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::runtime::Runtime;

/// This is the API Gateway container
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

        locator.register(Providers::default());
        locator.register::<HttpsClient>(builder.build(HttpsConnector::from((http_connector, tls))));
        locator.register(PipelineRunner::new());
        locator.register(authentication::all());
        locator.register(balancer::all());

        KatalystEngine {
            state: RwLock::default(),
            locator: locator,
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

#[derive(Default)]
pub struct Katalyst {
    engine: Arc<KatalystEngine>,
}

impl Katalyst {
    #[inline]
    pub fn engine(&self) -> Arc<KatalystEngine> {
        self.engine.clone()
    }

    /// Load a configuration file
    pub fn load(&self, config_file: &str) -> Result<(), KatalystError> {
        let config = parsers::parse_file(config_file);
        self.engine.update_state(config.build(self.engine())?)?;
        Ok(())
    }

    /// Start the API Gateway
    #[inline]
    pub fn run(&mut self) -> Result<(), KatalystError> {
        self.engine.run_service()?;
        self.engine.wait()?;
        Ok(())
    }
}
