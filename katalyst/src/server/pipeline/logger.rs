use crate::prelude::*;
use std::time::Instant;

pub(crate) fn log_request(guard: RequestContext) -> AsyncResult<()> {
    let ctx = ensure!(:guard.metadata());
    info!("Request started to {:?}", ctx.url);
    Ok(()).fut()
}

pub(crate) fn log_result(guard: RequestContext) -> RequestContext {
    if let Ok(ctx) = guard.metadata() {
        let duration = Instant::now().duration_since(ctx.started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        info!("Request processed in {:?}ms", total_ms);
    }
    guard
}

pub(crate) fn log_error(err: ModuleError) -> ModuleError {
    if let Ok(ctx) = err.context.metadata() {
        let duration = Instant::now().duration_since(ctx.started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        warn!("{} after {}ms", err.error, total_ms);
    }
    err
}
