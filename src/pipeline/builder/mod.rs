use super::*;
use crate::config::Gateway;
use hyper::{Request, Uri};

pub struct Builder {}

impl Pipeline for Builder {
    fn name(&self) -> &'static str {
        "builder"
    }

    fn process(&self, mut state: PipelineState, _config: &Gateway) -> PipelineState {
        {
            let route = state
                .matched_route
                .expect("Builder requires route to be matched already");
            let mut path = route.downstream.base_url.to_owned();
            path.push_str(&route.downstream.path);

            let url: Uri = path.parse().unwrap();
            let mut req = Request::builder();

            let up = state.upstream_request;
            let client_req = req
                .uri(url)
                .version(up.version())
                .method(up.method())
                .header("Proxied", "Test")
                .body(up.into_body())
                .unwrap();
            //*client_req.headers_mut() = *state.upstream_request.headers();

            state.upstream_request = Request::default();
            state.downstream_request = Some(client_req);
            state.matched_route = Some(route);
        }
        state
    }
}
