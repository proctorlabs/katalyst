/*!
Module traits and built in modules
*/

mod def;

pub(crate) mod authentication;
pub(crate) mod authorization;
pub(crate) mod balancer;
pub(crate) mod cache;
pub(crate) mod handlers;
pub(crate) mod plugins;

use crate::prelude::*;
use std::{collections::HashMap, sync::Arc};

pub use cache::CachedObject;
pub use def::*;

/// Module result type
pub type ModuleResultSync = Result<()>;
/// Async module result type
pub type ModuleResult = AsyncResult<()>;

/// Error type for modules
pub struct ModuleError {
    /// Error
    pub error: GatewayError,
    /// Context
    pub context: RequestContext,
}
