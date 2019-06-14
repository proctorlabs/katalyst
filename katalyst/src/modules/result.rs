use crate::prelude::*;

/// Module result type
pub(crate) type ModuleResultSync = Result<()>;
/// Async module result type
pub(crate) type ModuleResult = AsyncResult<()>;

pub(crate) struct ModuleError {
    pub error: GatewayError,
    pub context: RequestContext,
}
