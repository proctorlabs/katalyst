use super::*;
use futures::Future;
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
                        ctx.request = r.as_ref().clone().into_response();
                        err!(ctx, GatewayError::Done)
                    }
                    Err(_) => ok!(ctx),
                }),
        )
    }

    fn update_cache(&self, ctx: Context) -> ModuleResult {
        if !ctx.request.is_response() {
            return ok!(ctx);
        }
        let cache = try_fut!(
            ctx,
            ctx.katalyst
                .get_instance()
                .map_err(|_| GatewayError::InternalServerError)
        )
        .service
        .cache
        .clone();
        Box::new(ctx.preload().and_then(move |mut ctx| {
            ctx.request = match ctx.request {
                HttpRequest::LoadedResponse(_) => {
                    cache.set_key(&ctx.metadata.url.as_str(), CachedObject::from_response(&ctx.request).unwrap());
                    ctx.request
                }
                _ => ctx.request,
            };
            Ok(ctx)
        }))
    }
}
