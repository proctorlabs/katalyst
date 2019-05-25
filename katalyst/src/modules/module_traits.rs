use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use unstructured::Document;

pub type ModuleResultSync = std::result::Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

pub trait ModuleData {
    const MODULE_TYPE: ModuleType;
}

pub trait ModuleProvider: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &Document) -> Result<Module>;
}
