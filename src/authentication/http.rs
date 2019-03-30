use crate::app::HttpsClient;
use crate::authentication::*;
use crate::pipeline::*;
use futures::future::*;
use futures::stream::Stream;
use futures::Future;
use hyper::body::Body;
use hyper::Request;

#[derive(Default, Debug)]
pub struct HttpAuthenticatorBuilder {}

impl KatalystAuthenticatorBuilder for HttpAuthenticatorBuilder {
    fn name(&self) -> &'static str {
        "http"
    }

    fn build(&self, config: &AuthenticatorBuilder) -> Arc<KatalystAuthenticator> {
        if let Some(url) = &config.url {
            return Arc::new(HttpAuthenticator {
                url: url.to_string(),
            });
        }
        panic!("Invalid config!");
    }
}

#[derive(Default, Debug)]
pub struct HttpAuthenticator {
    url: String,
}

impl KatalystAuthenticator for HttpAuthenticator {
    fn authenticate(&self, mut state: PipelineState) -> AsyncPipelineResult {
        let client: Arc<HttpsClient> = state.engine.locate().unwrap();
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
                state.context.authentication = Some(auth);
                ok::<PipelineState, KatalystError>(state)
            }
            Err(_) => err::<PipelineState, KatalystError>(KatalystError::Forbidden),
        }))
    }
}
