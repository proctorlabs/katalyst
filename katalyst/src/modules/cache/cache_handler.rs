use super::*;
use futures::Future;
use hyper::{Body, Response};
use std::sync::Arc;

#[derive(Debug)]
pub struct DefaultCacheHandler;

impl ModuleProvider for DefaultCacheHandler {
    fn name(&self) -> &'static str {
        "cache_response"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(DefaultCacheHandler.into_module())
    }
}

impl CacheHandlerModule for DefaultCacheHandler {
    fn check_cache(&self, mut ctx: Context) -> ModuleResult {
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
                        ctx.request.set_response(resp);
                        ok!(ctx)
                    }
                    Err(_) => ok!(ctx),
                }),
        )
    }
}
