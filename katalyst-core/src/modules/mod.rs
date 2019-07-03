/*!
Module traits and built in modules
*/

mod def;
mod result;

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
pub(crate) use result::*;
