use crate::config::parsers;
use crate::config::Gateway;
use crate::service;
use std::sync::RwLock;

/// This is the API Gateway container
pub struct Katalyst {
    state: RwLock<Option<Gateway>>,
}

impl Katalyst {
    /// Create a new Katalyst instance
    pub fn new() -> Self {
        Katalyst {
            state: RwLock::new(None),
        }
    }

    /// Update the running configuration of the API Gateway.
    pub fn update_state(&self, new_state: Gateway) {
        let mut state = self.state.write().unwrap();
        *state = Option::Some(new_state);
    }

    /// Get a copy of the currently running API Gateway configuration.
    /// Will panic if the configuration has not yet been loaded.
    pub fn get_state(&self) -> Gateway {
        let state = self.state.read().unwrap();
        match (*state).clone() {
            Some(val) => val,
            None => panic!("Attempted to access state but configuration has not been loaded yet!"),
        }
    }

    /// Load a configuration file
    pub fn load(&self, config_file: &str) {
        let mut config = parsers::parse_file(config_file);
        self.update_state(config.build());
    }

    /// Start the API Gateway
    pub fn run(&self) {
        service::run_service(self.get_state());
    }
}
