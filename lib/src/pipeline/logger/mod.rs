use super::*;
use crate::config::Gateway;
use std::time::Instant;

pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineState {
        state
            .timestamps
            .insert("started".to_string(), Instant::now());
        debug!("Request received for URL {}", state.upstream_request.uri());
        state
    }

    fn post(&self, state: &PipelineState) {
        let started = state.timestamps["started"];
        let duration = Instant::now().duration_since(started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
    }

    fn error(&self, state: &PipelineState) {
        self.post(state);
    }
}
