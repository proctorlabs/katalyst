use std::any::Any;
use std::sync::Arc;

pub use crate::authentication::*;
pub use crate::context::Context;
pub use crate::error::KatalystError;
pub use crate::expression::*;
pub use crate::locator::Locatable;

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