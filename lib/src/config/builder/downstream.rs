use crate::state::Downstream;
use crate::templates::Providers;
use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DownstreamBuilder {
    base_url: String,
    path: String,
}

impl DownstreamBuilder {
    pub fn build(&self, providers: &Providers) -> Downstream {
        Downstream {
            base_url: self.base_url.to_owned(),
            path_parts: providers.process_template(&self.path.to_owned()),
        }
    }
}
