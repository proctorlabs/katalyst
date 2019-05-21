use super::*;
use crate::context::ResponseContainer;
use hyper::{Body, Response};
use std::sync::Arc;

#[derive(Debug)]
pub struct DefaultCacheHandler {}

impl Module for DefaultCacheHandler {
    fn name(&self) -> &'static str {
        "cache_response"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::CacheHandler]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>> {
        Ok(Arc::new(DefaultCacheHandler {}))
    }
}

impl ModuleDispatch for DefaultCacheHandler {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let instance = try_fut!(
            ctx,
            ctx.katalyst
                .get_instance()
                .map_err(|_| GatewayError::InternalServerError)
        );
        Box::new(
            instance
                .clone()
                .service
                .cache
                .get_key(&ctx.metadata.url.as_str())
                .then(|r| match r {
                    Ok(r) => {
                        let mut content = vec![];
                        content.clone_from_slice(r.as_slice());
                        let mut resp = Response::default();
                        *resp.status_mut() = http::status::StatusCode::OK;
                        *resp.body_mut() = Body::from(content);
                        ctx.response = ResponseContainer::new(resp);
                        ok!(ctx)
                    }
                    Err(_) => ok!(ctx),
                }),
        )
    }
}
