use crate::prelude::*;
use futures::future::*;
use hyper::{Body, Error, Response};

pub type HyperResult = Box<Future<Item = Response<Body>, Error = Error> + Send>;

pub fn map_result_to_hyper(res: ModuleResultSync) -> HyperResult {
    Box::new(match res {
        Ok(ctx) => ok::<Response<Body>, Error>(ctx.upstream.response.unwrap_or_default()),
        Err(e) => ok::<Response<Body>, Error>({
            let mut resp = Response::default();
            *resp.status_mut() = e.status_code();
            resp
        }),
    })
}
