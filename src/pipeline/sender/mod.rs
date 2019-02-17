use super::*;
use crate::config::Gateway;
use futures::future::*;
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
            //hyper::client::Client<hyper::client::connect::http::HttpConnector<hyper::client::connect::dns::TokioThreadpoolGaiResolver>>
            let builder = Client::builder();
            let client =
                builder.build(hyper::client::HttpConnector::new_with_tokio_threadpool_resolver());
            let res = client.request(request).wait().unwrap();
            state.upstream_response = res;
            state.downstream_request = None;
        }
        state
    }
}
