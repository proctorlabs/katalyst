use super::*;
use crate::app::HttpsClient;
use crate::pipeline::*;
use futures::future::*;
use futures::Future;
use std::sync::Arc;

impl HostDispatcher {
    pub fn prepare(&self, mut ctx: Context) -> PipelineResult {
        let config = ctx
            .engine
            .get_state()
            .context("Failed to get configuration")?;

        let balancer_lease = match config.hosts.get(&self.host) {
            Some(s) => s
                .servers
                .lease()
                .context("Failed to get lease from specified pool")?,
            None => {
                return Err(RequestFailure::NotFound);
            }
        };

        let transformer = self.transformer(&ctx, balancer_lease.to_string())?;
        ctx.detail.balancer_lease = Some(balancer_lease);

        let request = match ctx.upstream.request {
            Some(req) => req,
            None => return Err(RequestFailure::Internal),
        };

        let mut client_req = transformer.transform(request)?;
        ctx.upstream.request = None;
        add_forwarding_headers(&mut client_req.headers_mut(), &ctx.detail.remote_ip);
        strip_hop_headers(&mut client_req.headers_mut());
        ctx.downstream.request = Some(client_req);
        Ok(ctx)
    }

    pub fn send(mut ctx: Context) -> AsyncPipelineResult {
        let dsr = match ctx.downstream.request.take() {
            Some(s) => s,
            None => {
                return Box::new(err::<Context, RequestFailure>(RequestFailure::Internal));
            }
        };
        let client: Arc<HttpsClient> = match ctx.engine.locate() {
            Ok(c) => c,
            Err(_) => {
                return Box::new(err::<Context, RequestFailure>(RequestFailure::Internal));
            }
        };
        let res = client.request(dsr);
        Box::new(res.then(|response| match response {
            Ok(r) => {
                ctx.upstream.response = Some(r);
                ok::<Context, RequestFailure>(ctx)
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err::<Context, RequestFailure>(RequestFailure::GatewayTimeout)
            }
        }))
    }

    pub fn clean_response(mut ctx: Context) -> Context {
        if let Some(r) = &mut ctx.upstream.response {
            strip_hop_headers(r.headers_mut());
        }
        ctx
    }
}
