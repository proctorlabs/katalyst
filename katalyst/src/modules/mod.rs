/*!
Module traits and built in modules
*/

mod authentication;
mod authorization;
mod def;
mod handlers;
mod plugins;
mod registry;
mod result;

use crate::prelude::*;
use std::{collections::HashMap, sync::Arc};

pub(crate) mod balancer;
pub(crate) mod cache;
pub use cache::CachedObject;
pub use def::*;
pub use registry::ModuleRegistry;
pub use result::*;
