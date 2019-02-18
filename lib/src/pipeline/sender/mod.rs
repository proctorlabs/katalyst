use super::*;
use crate::config::Gateway;
use hyper::Client;

pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineState {
        {
            let request = state
                .downstream_request
                .expect("Sender requires a prepared request");
            let builder = Client::builder();
            let client =
                builder.build(hyper::client::HttpConnector::new_with_tokio_threadpool_resolver());
            let res = client.request(request);
            state.upstream_response = Box::new(res);
            state.downstream_request = None;
        }
        state
    }
}
