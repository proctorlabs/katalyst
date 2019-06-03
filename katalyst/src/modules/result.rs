use crate::prelude::*;

pub type ModuleResultSync = Result<()>;
pub type ModuleResult = AsyncResult<()>;

pub struct ModuleError {
    pub error: GatewayError,
    pub context: ContextGuard,
}

pub trait ModuleResultExt<T> {
    fn ctx<F>(self, ctx: F) -> std::result::Result<T, ModuleError>
    where
        F: FnOnce() -> ContextGuard;
}

impl<T> ModuleResultExt<T> for Result<T> {
    fn ctx<F>(self, ctx: F) -> std::result::Result<T, ModuleError>
    where
        F: FnOnce() -> ContextGuard,
    {
        self.map_err(|e| ModuleError { error: e, context: ctx() })
    }
}
