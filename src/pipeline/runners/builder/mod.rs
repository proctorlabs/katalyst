mod forwarding_headers;
mod hop_headers;
use crate::pipeline::*;

#[derive(Default)]
pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn prepare_request(&self, mut ctx: Context) -> PipelineResult {
        let config = ctx.engine.get_state()?;
        let downstream = match &ctx.context.matched_route {
            Some(route) => &route.downstream,
            None => {
                return Err(KatalystError::FeatureUnavailable);
            }
        };

        let balancer_lease = match config.hosts.get(&downstream.host) {
            Some(s) => s.servers.lease()?,
            None => {
                return Err(KatalystError::NotFound);
            }
        };

        let transformer = downstream.transformer(&ctx, balancer_lease.to_string())?;
        ctx.context.balancer_lease = Some(balancer_lease);

        let request = match ctx.upstream.request {
            Some(req) => req,
            None => return Err(KatalystError::FeatureUnavailable),
        };

        let mut client_req = transformer.transform(request)?;
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
