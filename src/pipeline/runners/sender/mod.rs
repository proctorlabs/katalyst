use crate::app::HttpsClient;
use crate::pipeline::*;
use futures::future::*;
use futures::Future;

#[derive(Default)]
pub struct Sender {}

impl Pipeline for Sender {
    fn name(&self) -> &'static str {
        "sender"
    }

    fn prepare_request_future(&self, mut ctx: Context) -> AsyncPipelineResult {
        let dsr = match ctx.downstream.request {
            Some(s) => {
                ctx.downstream.request = None;
                s
            }
            None => {
                return Box::new(err::<Context, KatalystError>(
                    KatalystError::FeatureUnavailable,
                ));
            }
        };
        let client: Arc<HttpsClient> = ctx.engine.locate().unwrap();
        let res = client.request(dsr);
        Box::new(res.then(|response| match response {
            Ok(r) => {
                ctx.upstream.response = Some(r);
                ok::<Context, KatalystError>(ctx)
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err::<Context, KatalystError>(KatalystError::GatewayTimeout)
            }
        }))
    }
}
