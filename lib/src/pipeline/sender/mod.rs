use super::*;
use crate::config::Gateway;
use futures::future::*;
use futures::Future;
use hyper::Client;

pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway,) -> PipelineResult {
        let builder = Client::builder();
        let client =
            builder.build(hyper::client::HttpConnector::new_with_tokio_threadpool_resolver());
        let dsr = state.downstream_request.unwrap();
        let res = client.request(dsr);
        state.downstream_request = None;
        Box::new(res.then(|r| {
            state.upstream_response = r.unwrap();
            ok::<PipelineState, PipelineError>(state)
        }))
    }

    fn make(&self) -> Box<Pipeline + Send + Sync> {
        Box::new(Sender {})
    }
}
