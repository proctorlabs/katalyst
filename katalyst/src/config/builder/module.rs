use super::*;
use crate::app::Katalyst;
use crate::error::GatewayError;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::string::String;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleBuilder<T: ModuleData> {
    #[serde(skip)]
    __module_type: PhantomData<T>,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(flatten)]
    pub config: unstructured::Document,
}

impl<T: ModuleData> Default for ModuleBuilder<T> {
    fn default() -> Self {
        ModuleBuilder {
            __module_type: PhantomData::default(),
            module: String::default(),
            config: unstructured::Document::Unit,
        }
    }
}

impl<T> Builder<Module> for ModuleBuilder<T>
where
    T: ModuleData,
{
    fn build(&self, engine: Arc<Katalyst>) -> Result<Module, GatewayError> {
        let module = engine.get_module(&self.module)?;
        Ok(module.build(T::MODULE_TYPE, engine.clone(), &self.config)?)
    }
}
