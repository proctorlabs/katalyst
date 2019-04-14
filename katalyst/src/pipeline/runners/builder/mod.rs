mod forwarding_headers;
mod hop_headers;

use crate::pipeline::*;
use crate::prelude::*;

#[derive(Default)]
pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn prepare_request(&self, mut ctx: Context) -> PipelineResult {
        let config = ctx
            .engine
            .get_state()
            .context("Failed to get configuration")?;
        let downstream = &ctx.detail.matched_route.with()?.downstream;

        let balancer_lease = match config.hosts.get(&downstream.host) {
            Some(s) => s
                .servers
                .lease()
                .context("Failed to get lease from specified pool")?,
            None => {
                return Err(RequestFailure::NotFound(ctx.lock()));
            }
        };

        let transformer = downstream
            .transformer(&ctx, balancer_lease.to_string())
            .context("Failed to create request")?;
        ctx.detail.balancer_lease = Some(balancer_lease);

        let request = match ctx.upstream.request {
            Some(req) => req,
            None => return Err(RequestFailure::Internal),
        };

        let mut client_req = transformer
            .transform(request)
            .context("Failed to create request")?;
        ctx.upstream.request = None;
        forwarding_headers::add_forwarding_headers(&mut client_req.headers_mut(), ctx.remote_addr);
        hop_headers::strip_hop_headers(&mut client_req.headers_mut());
        ctx.downstream.request = Some(client_req);
        Ok(ctx)
    }

    fn process_response(&self, mut ctx: Context) -> Context {
        if let Some(r) = &mut ctx.upstream.response {
            hop_headers::strip_hop_headers(r.headers_mut());
        }
        ctx
    }
}
