use crate::{app::Katalyst, context::*, modules::*};
use futures::{stream::Stream, Future};
use hyper::{body::Body, Request};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct HttpConfig {
    url: String,
}

#[derive(Default, Debug)]
pub struct HttpAuthenticatorBuilder;

impl ModuleProvider for HttpAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "http"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Module> {
        let c: HttpConfig = config.clone().try_into().map_err(|_| {
            GatewayError::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(HttpAuthenticator { url: c.url }.into_module())
    }
}

#[derive(Default, Debug)]
pub struct HttpAuthenticator {
    url: String,
}

impl AuthenticatorModule for HttpAuthenticator {
    fn authenticate(&self, guard: ContextGuard) -> AsyncResult<()> {
        let client = ensure_fut!(guard.katalyst()).get_client();
        let mut request = Request::builder();
        request.uri(&self.url.to_string());
        let res = client.request(request.body(Body::empty()).unwrap());
        Box::new(res.then(move |response| match response {
            Ok(resp) => {
                let (_, body) = resp.into_parts();
                let body = body
                    .map_err(|_| ())
                    .fold(vec![], |mut acc, chunk| {
                        acc.extend_from_slice(&chunk);
                        Ok(acc)
                    })
                    .and_then(|v| String::from_utf8(v).map_err(|_| ()))
                    .wait()
                    .unwrap();
                debug!("{}", body);
                let mut auth = Authentication::Authenticated { claims: HashMap::default() };
                auth.add_claim("KatalystAuthenticator".to_string(), "http".to_string());
                guard.set_authenticated(auth)
            }
            Err(_) => Err(GatewayError::Forbidden),
        }))
    }
}
