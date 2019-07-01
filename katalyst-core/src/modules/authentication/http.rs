use crate::{
    config::Builder,
    modules::*,
    util::{ClientRequestBuilder, CompiledClientRequest},
};
use futures::Future;

#[derive(Default, Debug)]
pub struct HttpAuthenticatorBuilder;

impl ModuleProvider for HttpAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "http"
    }

    fn build(&self, _: ModuleType, config: &unstructured::Document) -> Result<Module> {
        let request: ClientRequestBuilder = config.clone().try_into().map_err(|e| {
            err!(
                ConfigurationFailure,
                "Failed to parse HTTP authentication module configuration",
                e
            )
        })?;
        Ok(HttpAuthenticator { request: request.build()? }.into_module())
    }
}

#[derive(Debug)]
pub struct HttpAuthenticator {
    request: CompiledClientRequest,
}

impl AuthenticatorModule for HttpAuthenticator {
    fn authenticate(&self, guard: RequestContext) -> AsyncResult<()> {
        let request = ensure!(:self.request.prepare_request(&guard));
        let client = ensure!(:guard.katalyst()).get_client();
        Box::new(request.send_parse(&client).then(move |response| match response {
            Ok(_) => {
                let mut auth = Authentication::Authenticated { claims: HashMap::default() };
                auth.add_claim("KatalystCoreAuthenticator".to_string(), "http".to_string());
                guard.set_authentication(auth)
            }
            Err(e) => fail!(FORBIDDEN, "Access rejected due to downstream authenticator error", e),
        }))
    }
}
