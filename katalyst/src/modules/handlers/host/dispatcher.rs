use super::*;
use crate::prelude::*;
use futures::future::*;
use futures::Future;

impl HostDispatcher {
    pub fn prepare(&self, mut ctx: Context) -> ModuleResultSync {
        let config = try_req!(
            ctx,
            ctx.engine
                .get_instance()
                .map_err(|_| RequestFailure::Internal)
        );

        let balancer_lease = match config.hosts.get(&self.host) {
            Some(s) => try_req!(ctx, s.servers.lease().map_err(|_| RequestFailure::Internal)),
            None => {
                return Err(ctx.fail(RequestFailure::NotFound));
            }
        };

        let transformer = try_req!(ctx, self.transformer(&ctx, balancer_lease.to_string()));
        ctx.detail.balancer_lease = Some(balancer_lease);

        let request = match ctx.upstream.request.take() {
            Some(req) => req,
            None => return Err(ctx.fail(RequestFailure::Internal)),
        };

        let mut client_req = try_req!(ctx, transformer.transform(request));
        ctx.upstream.request = None;
        add_forwarding_headers(&mut client_req.headers_mut(), &ctx.detail.remote_ip);
        strip_hop_headers(&mut client_req.headers_mut());
        ctx.downstream.request = Some(client_req);
        Ok(ctx)
    }

    pub fn send(mut ctx: Context) -> ModuleResult {
        let dsr = match ctx.downstream.request.take() {
            Some(s) => s,
            None => {
                return err!(ctx, RequestFailure::Internal);
            }
        };
        let client = ctx.engine.get_client();
        let res = client.request(dsr);
        Box::new(res.then(|response| match response {
            Ok(r) => {
                ctx.upstream.response = Some(r);
                ok(ctx)
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err(ctx.fail(RequestFailure::GatewayTimeout))
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
