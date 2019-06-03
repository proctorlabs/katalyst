use crate::{app::Katalyst, context::*, expression::*, modules::*, *};
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
pub struct FileServerModule;

impl ModuleProvider for FileServerModule {
    fn name(&self) -> &'static str {
        "file_server"
    }

    fn build(
        &self,
        _: ModuleType,
        engine: Arc<Katalyst>,
        config: &unstructured::Document,
    ) -> Result<Module> {
        let c: FileServerConfig = config.clone().try_into().map_err(|_| {
            GatewayError::ConfigNotParseable("Host module configuration failed".into())
        })?;
        Ok(FileServerDispatcher {
            root_path: c.root_path,
            selector: engine.get_compiler().compile_template(Some(&c.selector))?,
        }
        .into_module())
    }
}

#[derive(Debug)]
pub struct FileServerDispatcher {
    pub root_path: String,
    pub selector: Expression,
}

impl RequestHandlerModule for FileServerDispatcher {
    fn dispatch(&self, guard: ContextGuard) -> ModuleResult {
        let path = ensure_fut!(self.selector.render(&guard));
        let mut full_path = PathBuf::from(&self.root_path);
        full_path.push(&path);
        send_file(guard.clone(), full_path)
    }
}

fn send_file(guard: ContextGuard, file: PathBuf) -> ModuleResult {
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
                file.extension().unwrap_or_default().to_str().unwrap_or_default(),
            )
            .unwrap_or("application/octet-stream");
            let hdrs = r.headers_mut();
            let hdr_val = HeaderValue::from_str(mime).unwrap();
            hdrs.append("Content-Type", hdr_val);
            guard.set_response(r).unwrap_or_default();
            ok(())
        }
        Err(_) => err(GatewayError::NotFound),
    }))
}
