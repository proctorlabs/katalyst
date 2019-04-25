use crate::app::HttpsClient;
use crate::app::KatalystEngine;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use futures::Future;
use hyper::body::Body;
use hyper::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct HttpConfig {
    url: String,
}

#[derive(Default, Debug)]
pub struct HttpAuthenticatorBuilder {}

impl Module for HttpAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "http"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Authenticator
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        config: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let c: HttpConfig = config.load()?;
        Ok(Arc::new(HttpAuthenticator { url: c.url }))
    }
}

#[derive(Default, Debug)]
pub struct HttpAuthenticator {
    url: String,
}

impl ModuleDispatch for HttpAuthenticator {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let client: Arc<HttpsClient> = ctx.engine.locate().unwrap();
        let mut request = Request::builder();
        request.uri(&self.url.to_string());
        let res = client.request(request.body(Body::empty()).unwrap());
        Box::new(res.then(|response| match response {
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
                let mut auth = KatalystAuthenticationInfo::default();
                auth.add_claim("KatalystAuthenticator".to_string(), "http".to_string());
                ctx.detail.authentication = Some(auth);
                ok(ctx)
            }
            Err(_) => err(ctx.fail(RequestFailure::Forbidden)),
        }))
    }
}
