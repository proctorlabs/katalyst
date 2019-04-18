use crate::pipeline::*;
use std::time::Instant;

#[derive(Default)]
pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn prepare_request(&self, ctx: Context) -> PipelineResult {
        info!("Request started to {:?}", ctx.detail.url);
        Ok(ctx)
    }

    fn process_response(&self, ctx: Context) -> Context {
        let duration = Instant::now().duration_since(ctx.timestamps.started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
        ctx
    }

    fn process_error(&self, err: RequestFailure) -> RequestFailure {
        warn!("Request failed with error: {}", err);
        err
    }
}
