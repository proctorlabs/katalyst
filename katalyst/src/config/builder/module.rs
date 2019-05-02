use super::*;
use crate::app::Katalyst;
use crate::error::ConfigurationFailure;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleBuilder<T: TypeId> {
    #[serde(skip)]
    pub __module_type: PhantomData<T>,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(flatten)]
    pub config: unstructured::Document,
}

impl<T: TypeId> Default for ModuleBuilder<T> {
    fn default() -> Self {
        ModuleBuilder {
            __module_type: PhantomData::default(),
            module: String::default(),
            config: unstructured::Document::Unit,
        }
    }
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
    fn build(&self, engine: Arc<Katalyst>) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let loader = ModuleConfigLoader {
            raw: self.config.clone(),
        };
        Ok(engine
            .get_module(&self.module, self.module_type())?
            .build(engine, &loader)?)
    }
}
