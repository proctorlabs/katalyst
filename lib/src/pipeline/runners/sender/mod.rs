use crate::config::Gateway;
use crate::pipeline::*;
use futures::future::*;
use futures::Future;

pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineResult {
        let dsr = state.downstream.request.unwrap();
        let res = state.client.request(dsr);
        state.downstream.request = None;
        Box::new(res.then(|r| {
            state.upstream.response = Some(r.unwrap());
            ok::<PipelineState, PipelineError>(state)
        }))
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Sender {})
    }
}
