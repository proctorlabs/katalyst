mod files;
mod host;

pub use files::*;
pub use host::*;

use crate::pipeline::*;
use crate::prelude::*;
use http::Method;

#[derive(Debug)]
pub enum Handler {
    Host(HostDispatcher),
    FileServer(FileServer),
}

pub trait Dispatchable {
    fn dispatch(&self, ctx: Context) -> AsyncPipelineResult;
}

impl Dispatchable for Handler {
    fn dispatch(&self, ctx: Context) -> AsyncPipelineResult {
        match self {
            Handler::Host(s) => s.dispatch(ctx),
            Handler::FileServer(s) => s.dispatch(ctx),
        }
    }
}
