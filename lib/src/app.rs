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
    pub fn new() -> Arc<KatalystEngine> {
        Arc::new(KatalystEngine {
            state: Arc::default(),
            providers: Providers::default(),
        })
    }
}

impl Katalyst for Arc<KatalystEngine> {
    fn update_state(&self, new_state: Gateway) {
        let mut state = self.state.write().unwrap();
        *state = Option::Some(new_state);
    }

    fn get_state(&self) -> Gateway {
        let state = self.state.read().unwrap();
        match (*state).clone() {
            Some(val) => val,
            None => panic!("Attempted to access state but configuration has not been loaded yet!"),
        }
    }

    fn load(&self, config_file: &str) {
        let mut config = parsers::parse_file(config_file);
        self.update_state(config.build(&self.providers));
    }

    fn run(&self) {
        service::run_service(self.clone());
    }
}

pub trait Katalyst {
    /// Update the running configuration of the API Gateway.
    fn update_state(&self, new_state: Gateway);

    /// Get a copy of the currently running API Gateway configuration.
    /// Will panic if the configuration has not yet been loaded.
    fn get_state(&self) -> Gateway;

    /// Load a configuration file
    fn load(&self, config_file: &str);

    /// Start the API Gateway
    fn run(&self);
}
