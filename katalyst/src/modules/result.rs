use crate::prelude::*;

/// Module result type
pub type ModuleResultSync = Result<()>;
/// Async module result type
pub type ModuleResult = AsyncResult<()>;

pub(crate) struct ModuleError {
    pub error: GatewayError,
    pub context: RequestContext,
}
