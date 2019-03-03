use crate::config::Gateway;
use std::error::Error;
use std::fmt;
use std::net::AddrParseError;
use std::sync;

#[derive(Debug)]
pub enum KatalystError {
    StateUpdateFailure,
    StateUnavailable,
    ConfigFailure,
    Unavailable,
    ConfigParseError,
}

impl Error for KatalystError {}

impl fmt::Display for KatalystError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KatalystError::ConfigFailure => write!(f, "Configuration failure occurred"),
            KatalystError::StateUpdateFailure => write!(f, "Unable to update internal state"),
            KatalystError::StateUnavailable => write!(f, "State is currently unavailable"),
            KatalystError::Unavailable => write!(f, "Feature unavailable"),
            KatalystError::ConfigParseError => write!(f, "Failed to parse configuration"),
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

impl From<AddrParseError> for KatalystError {
    fn from(_: AddrParseError) -> Self {
        KatalystError::ConfigParseError
    }
}
