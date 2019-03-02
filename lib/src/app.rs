use crate::config::parsers;
use crate::config::Gateway;
use crate::service;
use crate::templates::Providers;
use std::sync::Arc;
use std::sync::RwLock;

/// This is the API Gateway container
pub struct KatalystEngine {
    state: Arc<RwLock<Option<Gateway>>>,
    providers: Providers,
}

impl KatalystEngine {
    /// Update the running configuration of the API Gateway.
    pub fn update_state(&self, new_state: Gateway) {
        let mut state = self.state.write().unwrap();
        *state = Option::Some(new_state);
    }

    /// Get a copy of the currently running API Gateway configuration.
    /// Will panic if the configuration has not yet been loaded.
    pub fn get_state(&self) -> Result<Gateway, &'static str> {
        let state = self.state.read().unwrap();
        match state.clone() {
            Some(val) => Ok(val),
            None => Err("Attempted to access state but configuration has not been loaded yet!"),
        }
    }
}

pub struct Katalyst {
    engine: Arc<KatalystEngine>,
}

impl Katalyst {
    pub fn engine(&self) -> Arc<KatalystEngine> {
        self.engine.clone()
    }

    /// Load a configuration file
    pub fn load(&self, config_file: &str) {
        let mut config = parsers::parse_file(config_file);
        self.engine
            .update_state(config.build(&self.engine.providers));
    }

    /// Start the API Gateway
    pub fn run(&self) {
        service::run_service(self.engine.clone());
    }
}

impl Default for Katalyst {
    fn default() -> Self {
        Katalyst {
            engine: Arc::new(KatalystEngine {
                state: Arc::default(),
                providers: Providers::default(),
            }),
        }
    }
}
