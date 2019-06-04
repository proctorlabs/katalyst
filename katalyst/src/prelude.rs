/*!
This module provides a "prelude" useful for extending Katalyst functionality
*/

pub use crate::{
    app::Katalyst,
    context::*,
    error::{GatewayError::*, *},
    expression::*,
    modules::*,
};
pub(crate) use crate::{parser::*, *};
pub(crate) use futures::prelude::*;
pub(crate) use std::sync::Arc;

#[macro_export]
macro_rules! ensure_fut {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(e) => return Box::new(err(e)),
        }
    };
}

#[macro_export]
macro_rules! module_unwrap {
    ($name:ident, $mt:expr) => {
        Arc::new(match $mt {
            Module::$name(mtch) => mtch,
            _ => return Err(GatewayError::FeatureUnavailable),
        })
    };
}
