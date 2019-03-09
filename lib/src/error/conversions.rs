use super::KatalystError;
use crate::config::Gateway;
use std::net::AddrParseError;
use std::sync;

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
