mod authentication;
mod dispatcher;
mod logger;
mod matcher;

use super::Pipeline;
use crate::prelude::*;
use authentication::Authenticator;
use dispatcher::Dispatcher;
use logger::Logger;
use matcher::Matcher;
use std::sync::Arc;

pub fn all() -> Arc<[Arc<Pipeline>]> {
    Arc::new([
        Logger::arc(),
        Matcher::arc(),
        Authenticator::arc(),
        Dispatcher::arc(),
    ])
}
