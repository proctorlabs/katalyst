/*!
This module provides a "prelude" useful for extending Katalyst functionality
*/

use std::{any::Any, sync::Arc};

pub use crate::{app::Katalyst, context::*, error::*, expression::*, modules::*};
pub(crate) use crate::{parser::*, *};

pub(crate) trait KatalystCommonUtilities {
    fn arc() -> Arc<Self>
    where
        Self: Sized + Default,
    {
        Arc::new(Self::default())
    }
}

impl<T> KatalystCommonUtilities for T where T: Any {}

pub(crate) trait OptionUtilities<T> {
    fn with(&self, message: &'static str) -> Result<&T>;
    fn with_owned(self, message: &'static str) -> Result<T>;
}

impl<T> OptionUtilities<T> for Option<T>
where
    T: Any,
{
    fn with(&self, message: &'static str) -> Result<&T> {
        match self {
            Some(t) => Ok(t),
            None => {
                Err(GatewayError::RequestFailed(http::StatusCode::INTERNAL_SERVER_ERROR, message))
            }
        }
    }

    fn with_owned(self, message: &'static str) -> Result<T> {
        match self {
            Some(t) => Ok(t),
            None => {
                Err(GatewayError::RequestFailed(http::StatusCode::INTERNAL_SERVER_ERROR, message))
            }
        }
    }
}
