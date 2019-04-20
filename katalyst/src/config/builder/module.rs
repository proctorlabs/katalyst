use super::*;
use crate::app::KatalystEngine;
use crate::error::ConfigurationFailure;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModuleBuilder<T: TypeId> {
    #[serde(skip)]
    pub __module_type: PhantomData<T>,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(flatten)]
    pub config: serde_json::Value,
}

impl<T> ModuleBuilder<T>
where
    T: TypeId,
{
    pub fn module_type(&self) -> &'static str {
        T::KEY
    }
}

impl<T> Builder<Arc<ModuleDispatch>> for ModuleBuilder<T>
where
    T: TypeId,
{
    fn build(
        &self,
        engine: Arc<KatalystEngine>,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let modules: Arc<Modules> = engine.locate()?;
        let loader = ModuleConfigLoader {
            raw: self.config.clone(),
        };
        Ok(modules
            .get(&self.module, self.module_type())?
            .build(engine, &loader)?)
    }
}
