use crate::pipeline::*;
use crate::*;
use futures::future::*;

#[derive(Default)]
pub struct Authenticator {}

impl Pipeline for Authenticator {
    fn name(&self) -> &'static str {
        "authenticator"
    }

    fn prepare_request_future(&self, ctx: Context) -> AsyncPipelineResult {
        let route = try_fut!(ctx.detail.route());
        match &route.authenticators {
            Some(state_authenticators) => {
                let authenticators = state_authenticators.clone();
                let mut result: AsyncPipelineResult = Box::new(ok(ctx));
                for a in authenticators.iter() {
                    result = Box::new(result.and_then({
                        let r = a.clone();
                        move |s| r.dispatch(s)
                    }));
                }
                result
            }
            None => Box::new(ok(ctx)),
        }
    }
}
