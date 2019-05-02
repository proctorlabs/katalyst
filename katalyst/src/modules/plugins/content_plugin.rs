use crate::app::Katalyst;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use futures::stream::Stream;
use hyper::Request;
use unstructured::Document;

#[derive(Debug)]
pub struct ContentPlugin {}

impl Module for ContentPlugin {
    fn name(&self) -> &'static str {
        "parse-content"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::Plugin
    }

    fn build(
        &self,
        _: Arc<Katalyst>,
        _: &ModuleConfigLoader,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(ContentPlugin {}))
    }
}

impl ModuleDispatch for ContentPlugin {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let req = try_fut!(
            ctx,
            ctx.upstream.request.take().ok_or(RequestFailure::Internal)
        );
        let (parts, body) = req.into_parts();
        let format = Format::content_type(
            parts
                .headers
                .get("Content-Type")
                .map(|h| h.to_str().unwrap_or_default()),
        );
        match format {
            Format::Default => {
                ctx.upstream.request = Some(Request::from_parts(parts, body));
                ok!(ctx)
            }
            _ => Box::new(
                body.concat2()
                    .and_then(|body| {
                        let data = Parser::from_slice::<Document>(&body, format);
                        Ok((data, body))
                    })
                    .then(|res| match res {
                        Ok((Ok(data), body)) => {
                            ctx.set_extension_data(data);
                            ctx.upstream.request =
                                Some(Request::from_parts(parts, hyper::Body::from(body)));
                            Ok(ctx)
                        }
                        _ => Ok(ctx),
                    }),
            ),
        }
    }
}
