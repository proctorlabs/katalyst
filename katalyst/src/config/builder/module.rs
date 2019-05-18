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
pub struct ModuleBuilder<T: ModuleProvider> {
    #[serde(skip)]
    __module_type: PhantomData<T>,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(flatten)]
    pub config: unstructured::Document,
}

impl<T: ModuleProvider> Default for ModuleBuilder<T> {
    fn default() -> Self {
        ModuleBuilder {
            __module_type: PhantomData::default(),
            module: String::default(),
            config: unstructured::Document::Unit,
        }
    }
}

impl<T: ModuleProvider> Builder<T::ModuleImplType> for ModuleBuilder<T>
where
    T: ModuleProvider,
{
    fn build(&self, engine: Arc<Katalyst>) -> Result<T::ModuleImplType, GatewayError> {
        let module = engine.get_module(&self.module)?;
        if !module.supported_hooks().contains(&T::MODULE_TYPE) {
            return Err(GatewayError::InvalidResource);
        }
        Ok(T::build(module, engine, &self.config)?)
    }
}
