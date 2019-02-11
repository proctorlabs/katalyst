use crate::config::parsers;
use crate::config::Gateway;
use crate::service;
use std::sync::RwLock;

pub struct Katalyst {
    state: RwLock<Option<Gateway>>,
}

impl Katalyst {
    pub fn new() -> Self {
        Katalyst {
            state: RwLock::new(None),
        }
    }

    pub fn update_state(&self, new_state: Gateway) {
        let mut state = self.state.write().unwrap();
        *state = Option::Some(new_state);
    }

    pub fn get_state(&self) -> Gateway {
        let state = self.state.read().unwrap();
        match (*state).clone() {
            Some(val) => val,
            None => panic!("Attempted to access state but configuration has not been loaded yet!"),
        }
    }

    // Load some config file, placeholder for now...
    pub fn load(&self, config_file: &str) {
        let mut config = parsers::parse_file(config_file);
        self.update_state(config.build());
    }

    pub fn run(&self) {
        service::run_service(self.get_state());
    }
}
