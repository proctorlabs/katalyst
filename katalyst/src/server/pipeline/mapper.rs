use super::*;
use futures::future::*;
use hyper::{Body, Error, Response};

pub type HyperResult = Box<Future<Item = Response<Body>, Error = Error> + Send>;

pub fn map_result_to_hyper(res: PipelineResultSync) -> HyperResult {
    Box::new(match res {
        Ok(ctx) => ok::<Response<Body>, Error>(ctx.take_response().unwrap()),
        Err(e) => ok::<Response<Body>, Error>({
            let mut resp = Response::default();
            *resp.status_mut() = e.error.status_code();
            resp
        }),
    })
}
