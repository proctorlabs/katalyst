use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use unstructured::Document;

pub type ModuleResultSync = std::result::Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

#[derive(PartialEq, Debug)]
pub enum ModuleType {
    Authenticator,
    Authorizer,
    RequestHandler,
    Plugin,
    CacheProvider,
    CacheHandler,
}

pub trait ModuleProvider: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &Document) -> Result<Module>;
}

pub trait CacheProvider: Send + Sync + Debug {
    fn get_key(&self, key: &str) -> Box<Future<Item = Arc<Vec<u8>>, Error = GatewayError> + Send>;

    fn set_key(
        &self,
        key: &str,
        val: Vec<u8>,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send>;
}

pub trait RequestHook: Send + Sync + Debug {
    fn run(&self, ctx: Context) -> ModuleResult;
}

pub enum Module {
    Authenticator(Arc<dyn RequestHook>),
    Authorizer(Arc<dyn RequestHook>),
    RequestHandler(Arc<dyn RequestHook>),
    Plugin(Arc<dyn RequestHook>),
    CacheProvider(Arc<dyn CacheProvider>),
    CacheHandler(Arc<dyn RequestHook>),
}
