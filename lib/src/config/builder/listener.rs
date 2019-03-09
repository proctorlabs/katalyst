use crate::state::Listener;
use crate::templates::Providers;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListenerBuilder {
    pub interface: String,
}

impl ListenerBuilder {
    pub fn build(&self, _providers: &Providers) -> Listener {
        Listener {
            interface: self.interface.to_owned(),
        }
    }
}
