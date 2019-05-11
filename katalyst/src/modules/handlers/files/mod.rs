use crate::app::Katalyst;
use crate::context::*;
use crate::expression::*;
use crate::modules::*;
use crate::*;
use futures::future::*;
use http::header::HeaderValue;
use hyper::{Body, Response};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct FileServerConfig {
    root_path: String,
    selector: String,
}

#[derive(Debug)]
pub struct FileServerModule {}

impl Module for FileServerModule {
    fn name(&self) -> &'static str {
        "file_server"
    }

    fn supported_hooks(&self) -> Vec<ModuleType> {
        vec![ModuleType::RequestHandler]
    }

    fn build_hook(
        &self,
        _: ModuleType,
        engine: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        let c: FileServerConfig = config.clone().try_into().map_err(|_| {
            ConfigurationFailure::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(Arc::new(FileServerDispatcher {
            root_path: c.root_path,
            selector: engine.get_compiler().compile_template(Some(&c.selector))?,
        }))
    }
}

#[derive(Debug)]
pub struct FileServerDispatcher {
    pub root_path: String,
    pub selector: Expression,
}

impl ModuleDispatch for FileServerDispatcher {
    fn dispatch(&self, ctx: Context) -> ModuleResult {
        let path = try_fut!(ctx, self.selector.render(&ctx));
        let mut full_path = PathBuf::from(&self.root_path);
        full_path.push(&path);
        send_file(ctx, full_path)
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
            ctx.response = ResponseContainer::new(r);
            ok(ctx)
        }
        Err(_) => err(ctx.fail(RequestFailure::NotFound)),
    }))
}
