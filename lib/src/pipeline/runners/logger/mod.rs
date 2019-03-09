use crate::pipeline::*;
use std::time::Instant;

#[derive(Default)]
pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn process_result(&self, mut state: PipelineState) -> PipelineResult {
        state
            .context
            .timestamps
            .insert("started".to_string(), Instant::now());
        Ok(state)
    }

    fn post(&self, state: &PipelineState) {
        let started = state.context.timestamps["started"];
        let duration = Instant::now().duration_since(started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
    }

    fn error(&self, _: &KatalystError) {
        warn!("Pipeline processing failed!");
    }
}
