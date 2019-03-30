use super::KatalystError;
use crate::state::KatalystState;
use std::net::AddrParseError;
use std::sync;
use std::sync::Arc;

impl From<sync::PoisonError<sync::RwLockWriteGuard<'_, Arc<KatalystState>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockWriteGuard<Arc<KatalystState>>>) -> Self {
        KatalystError::StateUpdateFailure
    }
}

impl From<sync::PoisonError<sync::RwLockReadGuard<'_, Arc<KatalystState>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockReadGuard<Arc<KatalystState>>>) -> Self {
        KatalystError::StateUnavailable
    }
}

impl From<AddrParseError> for KatalystError {
    fn from(_: AddrParseError) -> Self {
        KatalystError::ConfigParseError
    }
}

impl From<regex::Error> for KatalystError {
    fn from(_: regex::Error) -> Self {
        KatalystError::ConfigFailure
    }
}
