use super::*;
use crate::config::Gateway;
use std::time::Instant;

pub struct Logger {}

#[allow(unused_variables, unused_mut)]
impl Pipeline for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn process(&self, state: &mut PipelineState, config: &Gateway) -> bool {
        state
            .timestamps
            .insert("started".to_string(), Instant::now());
        println!("Request received for URL {}", state.req.uri());
        true
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
