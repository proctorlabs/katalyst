use crate::{
    app::Katalyst,
    config::builder::Builder,
    context::*,
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

    fn build(
        &self,
        _: ModuleType,
        kat: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Module> {
        let request: ClientRequestBuilder = config.clone().try_into().map_err(|_| {
            GatewayError::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(HttpAuthenticator { request: request.build(kat)? }.into_module())
    }
}

#[derive(Debug)]
pub struct HttpAuthenticator {
    request: CompiledClientRequest,
}

impl AuthenticatorModule for HttpAuthenticator {
    fn authenticate(&self, guard: RequestContext) -> AsyncResult<()> {
        let request = ensure_fut!(self.request.prepare_request(&guard));
        let client = ensure_fut!(guard.katalyst()).get_client();
        Box::new(request.send_parse(&client).then(move |response| match response {
            Ok(_) => {
                let mut auth = Authentication::Authenticated { claims: HashMap::default() };
                auth.add_claim("KatalystAuthenticator".to_string(), "http".to_string());
                guard.set_authentication(auth)
            }
            Err(_) => Err(GatewayError::Forbidden),
        }))
    }
}
