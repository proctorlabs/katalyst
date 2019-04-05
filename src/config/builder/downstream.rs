use super::Builder;
use crate::app::KatalystEngine;
use crate::error::KatalystError;
use crate::state::Downstream;
use crate::templates::Providers;
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DownstreamBuilder {
    host: String,
    path: String,
    method: Option<String>,
    query: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Builder<Downstream> for DownstreamBuilder {
    fn build(&self, engine: Arc<KatalystEngine>) -> Result<Downstream, KatalystError> {
        let providers = engine.locate::<Providers>()?;
        let method = match &self.method {
            Some(m) => Some(Method::from_bytes(m.to_uppercase().as_bytes()).unwrap()),
            None => None,
        };
        Ok(Downstream {
            host: self.host.to_owned(),
            path: providers.process_template(&self.path),
            method: method,
            query: providers.process_template_map(&self.query),
            headers: providers.process_template_map(&self.headers),
            body: providers.process_template_option(&self.body),
        })
    }
}
