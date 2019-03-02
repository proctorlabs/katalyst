use crate::config::Gateway;
use std::error::Error;
use std::fmt;
use std::sync;

#[derive(Debug)]
pub enum KatalystError {
    StateUpdateFailure,
    StateUnavailable,
    ConfigFailure,
    Unavailable,
}

impl Error for KatalystError {}

impl fmt::Display for KatalystError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KatalystError::ConfigFailure => write!(f, "Configuration failure occurred!"),
            KatalystError::StateUpdateFailure => write!(f, "Configuration failure occurred!"),
            KatalystError::StateUnavailable => write!(f, "Configuration failure occurred!"),
            KatalystError::Unavailable => write!(f, "Configuration failure occurred!"),
        }
    }
}

impl From<sync::PoisonError<sync::RwLockWriteGuard<'_, Option<Gateway>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockWriteGuard<Option<Gateway>>>) -> Self {
        KatalystError::StateUpdateFailure
    }
}

impl From<sync::PoisonError<sync::RwLockReadGuard<'_, Option<Gateway>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockReadGuard<Option<Gateway>>>) -> Self {
        KatalystError::StateUnavailable
    }
}
