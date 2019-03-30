use crate::pipeline::*;
use std::time::Instant;

#[derive(Default)]
pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn prepare_request(&self, mut state: PipelineState) -> PipelineResult {
        state
            .context
            .timestamps
            .insert("started".to_string(), Instant::now());
        match &state.upstream.request {
            Some(r) => info!("Request started to: {:?}", r.uri().path()),
            None => warn!("Request started with no request in context!!"),
        }
        Ok(state)
    }

    fn process_response(&self, state: PipelineState) -> PipelineState {
        let started = state.context.timestamps["started"];
        let duration = Instant::now().duration_since(started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
        state
    }

    fn process_error(&self, err: KatalystError) -> KatalystError {
        warn!("Request failed with error: {:?}", err);
        err
    }
}
