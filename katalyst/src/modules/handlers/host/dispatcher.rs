use super::*;
use crate::context::*;
use futures::{future::*, Future};

impl HostDispatcher {
    pub fn prepare(&self, guard: ContextGuard) -> ModuleResultSync {
        let config = guard.katalyst()?;
        let metadata = guard.metadata()?;

        let balancer_lease = match config.get_instance()?.hosts.get(&self.host) {
            Some(s) => s.servers.lease()?,
            None => {
                return Err(GatewayError::NotFound);
            }
        };

        let transformer = self.transformer(guard.clone(), balancer_lease.to_string())?;
        let lease_lock: &mut Option<Arc<String>> = &mut metadata.balancer_lease.lock();
        *lease_lock = Some(balancer_lease);

        let request = guard.take_request()?;

        let mut client_req = transformer.transform(request)?;
        add_forwarding_headers(&mut client_req.headers_mut(), &guard.metadata()?.remote_ip);
        strip_hop_headers(&mut client_req.headers_mut());
        guard.set_http_request(HttpRequest::new(client_req))?;
        Ok(())
    }

    pub fn send(guard: ContextGuard) -> ModuleResult {
        let dsr = ensure_fut!(guard.take_request());
        let client = ensure_fut!(guard.katalyst()).get_client();
        let res = client.request(dsr);
        Box::new(res.then(move |response| match response {
            Ok(r) => {
                guard.set_response(r).unwrap_or_default();
                ok(())
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err(GatewayError::GatewayTimeout)
            }
        }))
    }

    pub fn clean_response(guard: ContextGuard) -> Result<()> {
        let mut req = guard.take_http_request()?;
        if let HttpRequest::RawResponse(res) = &mut req {
            strip_hop_headers(&mut res.0.headers);
        }
        guard.set_http_request(req)?;
        Ok(())
    }
}
