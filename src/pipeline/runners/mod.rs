mod authentication;
mod builder;
mod logger;
mod matcher;
mod sender;

use super::Pipeline;
use crate::prelude::*;
use authentication::Authenticator;
use builder::Builder;
use logger::Logger;
use matcher::Matcher;
use sender::Sender;
use std::sync::Arc;

pub fn all() -> Arc<[Arc<Pipeline>]> {
    Arc::new([
        Logger::arc(),
        Matcher::arc(),
        Authenticator::arc(),
        Builder::arc(),
        Sender::arc(),
    ])
}
