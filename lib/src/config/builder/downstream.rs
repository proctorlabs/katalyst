use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::Downstream;
use crate::templates::Providers;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DownstreamBuilder {
    base_url: String,
    path: String,
}

impl DownstreamBuilder {
    pub fn build(&self, engine: Arc<KatalystEngine>) -> Result<Downstream, KatalystError> {
        let providers = engine.locate::<Providers>().unwrap();
        Ok(Downstream {
            base_url: self.base_url.to_owned(),
            path_parts: providers.process_template(&self.path.to_owned()),
        })
    }
}
