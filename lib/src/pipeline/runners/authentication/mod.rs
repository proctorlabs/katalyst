use crate::pipeline::*;

#[derive(Default)]
pub struct Authenticator {}

impl Pipeline for Authenticator {
    fn name(&self) -> &'static str {
        "authenticator"
    }

    fn prepare_request(&self, mut state: PipelineState) -> PipelineResult {
        let route = match &state.context.matched_route {
            Some(s) => s,
            None => return Err(KatalystError::Unavailable),
        };
        let mut atcs = vec![];
        match &route.authenticators {
            Some(authenticators) => {
                for a in authenticators {
                    atcs.push(&a.authenticator);
                }
            }
            None => return Ok(state),
        };
        for a in atcs {
            let result = a.authenticate(&state);
            if let Ok(auth_result) = result {
                state.context.authentication = Some(auth_result);
                return Ok(state);
            }
        }
        Err(KatalystError::Unauthorized)
    }
}
