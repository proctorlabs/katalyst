mod dispatcher;
mod transformers;
mod util;

use super::*;
use crate::expression::*;
use crate::prelude::*;
use futures::future::*;
use futures::Future;
use std::collections::HashMap;
use transformers::DownstreamTransformer;
pub use util::*;

#[derive(Debug)]
pub struct HostDispatcher {
    pub host: String,
    pub path: Expression,
    pub method: Option<Method>,
    pub query: Option<HashMap<String, Expression>>,
    pub headers: Option<HashMap<String, Expression>>,
    pub body: Option<Expression>,
}

impl Dispatchable for HostDispatcher {
    fn dispatch(&self, ctx: Context) -> AsyncPipelineResult {
        Box::new(
            result(self.prepare(ctx))
                .and_then(HostDispatcher::send)
                .map(HostDispatcher::clean_response),
        )
    }
}

impl HostDispatcher {
    pub fn transformer(
        &self,
        ctx: &Context,
        lease_str: String,
    ) -> Result<DownstreamTransformer, RequestFailure> {
        let mut uri = lease_str;
        uri.push_str(&self.path.render(ctx)?);
        if let Some(query) = &self.query {
            uri.push_str("?");
            for (key, val) in query.iter() {
                uri.push_str(&key);
                uri.push_str("=");
                uri.push_str(&val.render(&ctx)?);
                uri.push_str("&");
            }
            uri.truncate(uri.len() - 1);
        };

        let method = self.method.clone();

        let headers = match &self.headers {
            Some(h) => Some(
                h.iter()
                    .map(|(key, val)| Ok((key.to_string(), val.render(ctx)?)))
                    .collect::<Result<HashMap<String, String>, RequestFailure>>()?,
            ),
            None => None,
        };

        let body = match &self.body {
            Some(b) => Some(b.render(&ctx)?),
            None => None,
        };

        Ok(DownstreamTransformer {
            uri,
            method,
            headers,
            body,
        })
    }
}
