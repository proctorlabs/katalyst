use crate::app::KatalystEngine;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use hyper::Request;
use serde_json::Value;

#[derive(Debug)]
pub struct JsonPlugin {}

impl Module for JsonPlugin {
    fn name(&self) -> &'static str {
        "json"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Plugin
    }

    fn build(
        &self,
        _: Arc<KatalystEngine>,
        _: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(JsonPlugin {}))
    }
}

impl ModuleDispatch for JsonPlugin {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let req = try_fut!(ctx.upstream.request.take().ok_or(RequestFailure::Internal));
        let (parts, body) = req.into_parts();
        if let Some(header) = parts.headers.get("Content-Type") {
            if header != "application/json" {
                ctx.upstream.request = Some(Request::from_parts(parts, body));
                ok!(ctx)
            } else {
                Box::new(
                    body.concat2()
                        .and_then(|body| {
                            let data = serde_json::from_slice::<Value>(&body);
                            Ok((data, body))
                        })
                        .then(|res| match res {
                            Ok((Ok(data), body)) => {
                                println!("{}", data);
                                ctx.set_extension_data(data);

                                ctx.upstream.request =
                                    Some(Request::from_parts(parts, hyper::Body::from(body)));
                                Ok(ctx)
                            }
                            _ => Ok(ctx),
                        }),
                )
            }
        } else {
            ctx.upstream.request = Some(Request::from_parts(parts, body));
            ok!(ctx)
        }
    }
}
