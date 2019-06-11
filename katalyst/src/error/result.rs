use crate::prelude::*;

pub use futures::future::{done, err, ok};

pub type Result<T> = std::result::Result<T, GatewayError>;
pub type AsyncResult<T> = Box<Future<Item = T, Error = GatewayError> + Send>;

pub trait ResultExt<T> {
    fn fut(self) -> AsyncResult<T>;
}

impl<T: 'static + Send> ResultExt<T> for Result<T> {
    fn fut(self) -> AsyncResult<T> {
        Box::new(done(self))
    }
}
