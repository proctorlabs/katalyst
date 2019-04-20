use crate::app::HttpsClient;
use crate::app::KatalystEngine;
use crate::config::builder::AuthenticatorBuilder;
use crate::context::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use futures::Future;
use hyper::body::Body;
use hyper::Request;

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
        config: &ModuleConfig,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        match config {
            ModuleConfig::Authenticator(config) => match config {
                AuthenticatorBuilder::Http { url } => {
                    if let Some(url) = &url {
                        Ok(Arc::new(HttpAuthenticator {
                            url: url.to_string(),
                        }))
                    } else {
                        Err(ConfigurationFailure::InvalidResource)
                    }
                }
                _ => Err(ConfigurationFailure::InvalidResource),
            },
            _ => Err(ConfigurationFailure::InvalidResource),
        }
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
                ok::<Context, RequestFailure>(ctx)
            }
            Err(_) => err::<Context, RequestFailure>(RequestFailure::Forbidden),
        }))
    }
}
