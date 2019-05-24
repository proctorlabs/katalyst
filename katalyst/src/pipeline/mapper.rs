use crate::prelude::*;
use futures::future::*;
use hyper::{Body, Error, Response};

pub type HyperResult = Box<Future<Item = Response<Body>, Error = Error> + Send>;

pub fn map_result_to_hyper(res: ModuleResultSync) -> HyperResult {
    Box::new(match res {
        Ok(mut ctx) => ok::<Response<Body>, Error>(ctx.request.take_response()),
        Err(e) => ok::<Response<Body>, Error>({
            let mut resp = Response::default();
            *resp.status_mut() = e.error.status_code();
            resp
        }),
    })
}
