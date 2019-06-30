use hyper::{
    client::{connect::dns::TokioThreadpoolGaiResolver, HttpConnector},
    Body, Client,
};
use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use std::sync::Arc;

pub type HttpsClient = Client<HttpsConnector<HttpConnector<TokioThreadpoolGaiResolver>>, Body>;

/// Lightweight container around the underlying hyper client implementation.
/// This struct is thread safe and has a lightweight clone.
#[derive(Clone, Debug)]
pub struct ProxyClient(Arc<HttpsClient>);

impl Default for ProxyClient {
    fn default() -> Self {
        let builder = Client::builder();
        let mut http_connector = HttpConnector::new_with_tokio_threadpool_resolver();
        http_connector.enforce_http(false);
        let mut tls = ClientConfig::new();
        tls.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        ProxyClient(Arc::new(builder.build(HttpsConnector::from((http_connector, tls)))))
    }
}

impl ProxyClient {
    /// Retrieve the underlying hyper client from the proxy
    pub fn hyper_client(&self) -> Arc<HttpsClient> {
        self.0.clone()
    }
}
