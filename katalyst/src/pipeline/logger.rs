use crate::prelude::*;
use std::time::Instant;

pub fn log_request(ctx: Context) -> ModuleResult {
    info!("Request started to {:?}", ctx.metadata.url);
    ok!(ctx)
}

pub fn log_result(ctx: Context) -> Context {
    let duration = Instant::now().duration_since(ctx.metadata.started);
    let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
    debug!("Request processed in {:?}ms", total_ms);
    ctx
}

pub fn log_error(err: ModuleError) -> ModuleError {
    let duration = Instant::now().duration_since(err.context.metadata.started);
    let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
    warn!(
        "Request failed with error: {} after {:?}ms",
        err.error, total_ms
    );
    err
}
