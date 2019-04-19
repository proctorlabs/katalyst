use super::*;
use crate::app::HttpsClient;
use crate::pipeline::*;
use futures::future::*;
use futures::Future;
use http::header::HeaderValue;
use http::HeaderMap;
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
        let dsr = match ctx.downstream.request {
            Some(s) => {
                ctx.downstream.request = None;
                s
            }
            None => {
                return Box::new(err::<Context, RequestFailure>(RequestFailure::Internal));
            }
        };
        let client: Arc<HttpsClient> = ctx.engine.locate().unwrap();
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

    pub fn clean(mut ctx: Context) -> Context {
        if let Some(r) = &mut ctx.upstream.response {
            strip_hop_headers(r.headers_mut());
        }
        ctx
    }
}

lazy_static! {
    static ref HOP_HEADERS: Vec<&'static str> = vec![
        "Connection",
        "Keep-Alive",
        "Proxy-Authenticate",
        "Proxy-Authorization",
        "Te",
        "Trailers",
        "Transfer-Encoding",
        "Upgrade",
    ];
}

pub fn strip_hop_headers(headers: &mut HeaderMap) {
    for header in HOP_HEADERS.iter() {
        headers.remove(*header);
    }
}

fn add_forwarding_headers(headers: &mut HeaderMap, remote_ip: &str) {
    headers.remove("X-Forwarded-For");
    headers.remove("X-Forwarded-Proto");
    headers.remove("X-Forwarded-Port");
    if let Ok(header) = HeaderValue::from_str(remote_ip) {
        headers.append("X-Forwarded-For", header);
    }
}
