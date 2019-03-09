use crate::config::Gateway;
use crate::pipeline::*;
use std::time::Instant;

pub struct Logger {}

impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineResult {
        state
            .context
            .timestamps
            .insert("started".to_string(), Instant::now());
        self.ok(state)
    }

    fn post(&self, state: &PipelineState) {
        let started = state.context.timestamps["started"];
        let duration = Instant::now().duration_since(started);
        let total_ms = u64::from(duration.subsec_millis()) + (duration.as_secs() * 1000);
        debug!("Request processed in {:?}ms", total_ms);
    }

    fn error(&self, _: &PipelineError) {
        warn!("Pipeline processing failed!");
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Logger {})
    }
}
