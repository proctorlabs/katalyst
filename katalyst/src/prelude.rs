/*!
This module provides a "prelude" useful for extending Katalyst functionality
*/

use std::any::Any;
use std::sync::Arc;

pub use crate::app::Katalyst;
pub use crate::context::Context;
pub use crate::error::GatewayError;
pub use crate::expression::*;
pub use crate::modules::*;
pub(crate) use crate::parser::*;
pub(crate) use crate::*;

pub(crate) trait KatalystCommonUtilities {
    fn arc() -> Arc<Self>
    where
        Self: Sized + Default,
    {
        Arc::new(Self::default())
    }

    fn boxed() -> Box<Self>
    where
        Self: Sized + Default,
    {
        Box::new(Self::default())
    }
}

impl<T> KatalystCommonUtilities for T where T: Any {}

pub(crate) trait OptionUtilities<T> {
    fn with(&self, message: &'static str) -> Result<&T, GatewayError>;
    fn with_owned(self, message: &'static str) -> Result<T, GatewayError>;
}

impl<T> OptionUtilities<T> for Option<T>
where
    T: Any,
{
    fn with(&self, message: &'static str) -> Result<&T, GatewayError> {
        match self {
            Some(t) => Ok(t),
            None => Err(GatewayError::Other(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                message,
            )),
        }
    }

    fn with_owned(self, message: &'static str) -> Result<T, GatewayError> {
        match self {
            Some(t) => Ok(t),
            None => Err(GatewayError::Other(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                message,
            )),
        }
    }
}
