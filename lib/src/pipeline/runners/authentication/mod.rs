use crate::pipeline::*;
use futures::future::*;

#[derive(Default)]
pub struct Authenticator {}

impl Pipeline for Authenticator {
    fn name(&self) -> &'static str {
        "authenticator"
    }

    fn prepare_request_future(&self, mut state: PipelineState) -> AsyncPipelineResult {
        let route = match &state.context.matched_route {
            Some(s) => s,
            None => {
                return Box::new(err(KatalystError::FeatureUnavailable));
            }
        };
        let mut atcs = vec![];
        match &route.authenticators {
            Some(authenticators) => {
                for a in authenticators {
                    atcs.push(&a.authenticator);
                }
            }
            None => return Box::new(ok(state)),
        };
        for a in atcs {
            let result = a.authenticate(&state);
            if let Ok(auth_result) = result {
                state.context.authentication = Some(auth_result);
                return Box::new(ok(state));
            }
        }
        Box::new(err(KatalystError::Unauthorized))
    }
}
