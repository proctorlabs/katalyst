use crate::app::Katalyst;
use crate::context::*;
use crate::modules::*;
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

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::Plugin]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Ok(Arc::new(ContentPlugin {}))
    }
}

impl ModuleDispatch for ContentPlugin {
    fn dispatch(&self, mut ctx: Context) -> ModuleResult {
        let req = ctx.request.take();
        ctx.request = RequestContainer::Empty;
        let (parts, body) = req.into_parts();
        let format = Format::content_type(
            parts
                .headers
                .get("Content-Type")
                .map(|h| h.to_str().unwrap_or_default()),
        );
        match format {
            Format::Default => {
                ctx.request = RequestContainer::new(Request::from_parts(parts, body));
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
                            ctx.request = RequestContainer::new(Request::from_parts(
                                parts,
                                hyper::Body::from(body),
                            ));
                            Ok(ctx)
                        }
                        _ => Ok(ctx),
                    }),
            ),
        }
    }
}
