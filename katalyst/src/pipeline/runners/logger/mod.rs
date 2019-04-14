use crate::pipeline::*;
use std::time::Instant;

#[derive(Default)]
pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn prepare_request(&self, mut ctx: Context) -> PipelineResult {
        ctx.detail
            .timestamps
            .insert("started".to_string(), Instant::now());
        match &ctx.upstream.request {
            Some(r) => info!("Request started to: {:?}", r.uri().path()),
            None => warn!("Request started with no request in context!!"),
        }
        Ok(ctx)
    }

    fn process_response(&self, ctx: Context) -> Context {
        let started = ctx.detail.timestamps["started"];
        let duration = Instant::now().duration_since(started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
        ctx
    }

    fn process_error(&self, err: RequestFailure) -> RequestFailure {
        warn!("Request failed with error: {}", err);
        err
    }
}
