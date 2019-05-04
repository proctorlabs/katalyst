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
pub struct ModuleBuilder<T: PhantomModuleData> {
    #[serde(skip)]
    __module_type: PhantomData<T>,
    #[serde(rename = "type")]
    pub module: String,
    #[serde(flatten)]
    pub config: unstructured::Document,
}

impl<T: PhantomModuleData> Default for ModuleBuilder<T> {
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
    T: PhantomModuleData,
{
    pub fn module_type(&self) -> ModuleType {
        T::MODULE_TYPE
    }
}

impl<T> Builder<Arc<ModuleDispatch>> for ModuleBuilder<T>
where
    T: PhantomModuleData,
{
    fn build(&self, engine: Arc<Katalyst>) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let module = engine.get_module(&self.module)?;
        if !module.supported_hooks().contains(&self.module_type()) {
            return Err(ConfigurationFailure::InvalidResource);
        }
        Ok(engine
            .get_module(&self.module)?
            .build_hook(self.module_type(), engine, &self.config)?)
    }
}
