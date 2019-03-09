use crate::config::Gateway;
use crate::pipeline::*;
use futures::future::*;
use futures::Future;

#[derive(Default)]
pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> AsyncPipelineResult {
        let dsr = state.downstream.request.unwrap();
        let res = state.client.request(dsr);
        state.downstream.request = None;
        Box::new(res.then(|r| {
            state.upstream.response = Some(r.unwrap());
            ok::<PipelineState, KatalystError>(state)
        }))
    }
}
