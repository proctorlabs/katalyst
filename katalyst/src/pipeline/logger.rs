use crate::prelude::*;
use std::time::Instant;

pub fn log_request(ctx: Context) -> ModuleResult {
    info!("Request started to {:?}", ctx.detail.url);
    ok!(ctx)
}

pub fn log_result(ctx: Context) -> Context {
    let duration = Instant::now().duration_since(ctx.timestamps.started);
    let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
    debug!("Request processed in {:?}ms", total_ms);
    ctx
}

pub fn log_error(err: RequestFailure) -> RequestFailure {
    warn!("Request failed with error: {}", err);
    err
}
