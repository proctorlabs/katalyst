use crate::app::KatalystEngine;
use crate::config::builder::HandlerBuilder;
use crate::expression::*;
use crate::modules::*;
use crate::prelude::*;
use futures::future::*;
use http::header::HeaderValue;
use hyper::{Body, Response};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileServerModule {}

impl Module for FileServerModule {
    fn name(&self) -> &'static str {
        "file_server"
    }

    fn module_type(&self) -> ModuleType {
        ModuleType::RequestHandler
    }

    fn build(
        &self,
        engine: Arc<KatalystEngine>,
        config: &ModuleConfig,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        match config {
            ModuleConfig::RequestHandler(config) => match config {
                HandlerBuilder::FileServer {
                    root_path,
                    selector,
                } => Ok(Arc::new(FileServerDispatcher {
                    root_path: root_path.to_owned(),
                    selector: engine
                        .locate::<Compiler>()?
                        .compile_template(Some(selector))?,
                })),
                _ => Err(ConfigurationFailure::InvalidResource),
            },
            _ => Err(ConfigurationFailure::InvalidResource),
        }
    }
}

#[derive(Debug)]
pub struct FileServerDispatcher {
    pub root_path: String,
    pub selector: Expression,
}

impl ModuleDispatch for FileServerDispatcher {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        let path = self.selector.render(&ctx);
        if let Ok(path) = path {
            let mut full_path = PathBuf::from(&self.root_path);
            full_path.push(&path);
            send_file(ctx, full_path)
        } else {
            Box::new(err::<Context, RequestFailure>(RequestFailure::Internal))
        }
    }
}

fn send_file(mut ctx: Context, file: PathBuf) -> ModuleResult {
    let result = Box::new(
        tokio_fs::file::File::open(file.to_str().unwrap_or_default().to_string()).and_then(
            |file| {
                let buf: Vec<u8> = Vec::new();
                tokio_io::io::read_to_end(file, buf)
                    .and_then(|item| Ok(Response::<Body>::new(item.1.into())))
            },
        ),
    );
    Box::new(result.then(move |result| match result {
        Ok(mut r) => {
            let mime = mime_guess::get_mime_type_str(
                file.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            )
            .unwrap_or("application/octet-stream");
            let hdrs = r.headers_mut();
            let hdr_val = HeaderValue::from_str(mime).unwrap();
            hdrs.append("Content-Type", hdr_val);
            ctx.upstream.response = Some(r);
            ok::<Context, RequestFailure>(ctx)
        }
        Err(_) => err::<Context, RequestFailure>(RequestFailure::NotFound),
    }))
}
