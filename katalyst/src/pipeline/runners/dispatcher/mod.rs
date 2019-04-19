use crate::instance::handlers::Dispatchable;
use crate::pipeline::*;
use futures::future::*;

#[derive(Default)]
pub struct Dispatcher {}

impl Pipeline for Dispatcher {
    fn name(&self) -> &'static str {
        "dispatcher"
    }

    fn prepare_request_future(&self, ctx: Context) -> AsyncPipelineResult {
        match ctx.detail.route() {
            Ok(r) => Box::new(r.clone().handler.dispatch(ctx)),
            Err(e) => Box::new(err(e)),
        }
    }
}
