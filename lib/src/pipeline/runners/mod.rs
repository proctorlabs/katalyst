mod builder;
mod logger;
mod matcher;
mod sender;

use super::Pipeline;
use builder::Builder;
use logger::Logger;
use matcher::Matcher;
use sender::Sender;
use std::sync::Arc;

pub fn all() -> Arc<[Arc<Pipeline + Send + Sync>]> {
    Arc::new([
        Arc::new(Logger {}),
        Arc::new(Matcher {}),
        Arc::new(Builder {}),
        Arc::new(Sender {}),
    ])
}
