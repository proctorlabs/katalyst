use super::*;
use crate::expression::*;
use crate::prelude::*;
use futures::future::*;
use hyper::{Body, Response};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileServer {
    pub root_path: String,
    pub selector: Expression,
}

impl Dispatchable for FileServer {
    fn dispatch(&self, ctx: Context) -> AsyncPipelineResult {
        let path = self.selector.render(&ctx);
        if let Ok(path) = path {
            let mut full_path = PathBuf::from(&self.root_path);
            full_path.push(&path);
            send_file(ctx, full_path.to_str().unwrap_or_default().to_string())
        } else {
            Box::new(err::<Context, RequestFailure>(RequestFailure::Internal))
        }
    }
}

fn send_file(mut ctx: Context, filename: String) -> AsyncPipelineResult {
    let result = Box::new(tokio_fs::file::File::open(filename).and_then(|file| {
        let buf: Vec<u8> = Vec::new();
        tokio_io::io::read_to_end(file, buf)
            .and_then(|item| Ok(Response::<Body>::new(item.1.into())))
    }));
    Box::new(result.then(|result| match result {
        Ok(r) => {
            ctx.upstream.response = Some(r);
            ok::<Context, RequestFailure>(ctx)
        }
        Err(_) => err::<Context, RequestFailure>(RequestFailure::NotFound),
    }))
}
