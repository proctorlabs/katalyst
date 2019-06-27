use crate::prelude::*;

pub(crate) use futures::future::{done, err, ok};

/// Katalyst result type
pub type Result<T> = std::result::Result<T, GatewayError>;
/// Katalyst async result type
pub type AsyncResult<T> = Box<Future<Item = T, Error = GatewayError> + Send>;

pub(crate) trait ResultExt<T> {
    fn fut(self) -> AsyncResult<T>;
}

impl<T: 'static + Send> ResultExt<T> for Result<T> {
    fn fut(self) -> AsyncResult<T> {
        Box::new(done(self))
    }
}
