use crate::prelude::*;

pub type ModuleResultSync = Result<()>;
pub type ModuleResult = AsyncResult<()>;

pub struct ModuleError {
    pub error: GatewayError,
    pub context: RequestContext,
}

pub trait ModuleResultExt<T> {
    fn ctx<F>(self, ctx: F) -> std::result::Result<T, ModuleError>
    where
        F: FnOnce() -> RequestContext;
}

impl<T> ModuleResultExt<T> for Result<T> {
    fn ctx<F>(self, ctx: F) -> std::result::Result<T, ModuleError>
    where
        F: FnOnce() -> RequestContext,
    {
        self.map_err(|e| ModuleError { error: e, context: ctx() })
    }
}
