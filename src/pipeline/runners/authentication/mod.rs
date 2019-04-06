use crate::authentication::KatalystAuthenticator;
use crate::pipeline::*;
use futures::future::*;
use std::sync::Arc;

#[derive(Default)]
pub struct Authenticator {}

impl Pipeline for Authenticator {
    fn name(&self) -> &'static str {
        "authenticator"
    }

    fn prepare_request_future(&self, ctx: Context) -> AsyncPipelineResult {
        let route = match &ctx.context.matched_route {
            Some(s) => s,
            None => {
                return Box::new(err(KatalystError::FeatureUnavailable));
            }
        };
        match &route.authenticators {
            Some(state_authenticators) => {
                let authenticators: Vec<Arc<KatalystAuthenticator>> = state_authenticators
                    .iter()
                    .map(|a| a.authenticator.clone())
                    .collect();
                let mut result: AsyncPipelineResult = Box::new(ok(ctx));
                for a in authenticators.iter() {
                    result = Box::new(result.and_then({
                        let r = a.clone();
                        move |s| r.authenticate(s)
                    }));
                }
                result
            }
            None => Box::new(ok(ctx)),
        }
    }
}
