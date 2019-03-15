use super::Builder;
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
    host: String,
    path: String,
}

impl Builder<Downstream> for DownstreamBuilder {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Downstream, KatalystError> {
        let providers = engine.locate::<Providers>().unwrap();
        Ok(Downstream {
            host: self.host.to_owned(),
            path_parts: providers.process_template(&self.path.to_owned()),
        })
    }
}
