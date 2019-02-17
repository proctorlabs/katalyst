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
        println!("Request received for URL {}", state.upstream_request.uri());
        state
    }

    fn post(&self, state: &PipelineState) {
        let started = state.timestamps.get("started").unwrap();
        println!(
            "Request processed in {}",
            Instant::now().duration_since(*started).subsec_micros()
        );
    }

    fn error(&self, state: &PipelineState) {
        self.post(state);
    }
}
