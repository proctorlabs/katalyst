use crate::pipeline::*;
use crate::*;

#[derive(Default)]
pub struct Dispatcher {}

impl Pipeline for Dispatcher {
    fn name(&self) -> &'static str {
        "dispatcher"
    }

    fn prepare_request_future(&self, ctx: Context) -> AsyncPipelineResult {
        let r = try_fut!(ctx.detail.route()).clone();
        Box::new(r.handler.dispatch(ctx))
    }
}
