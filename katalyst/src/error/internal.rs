use crate::instance::Instance;
use std::net::AddrParseError;
use std::sync;
use std::sync::Arc;

/// KatalystError
#[derive(Debug, Fail)]
pub enum KatalystError {
    #[fail(display = "Unable to update internal state")]
    StateUpdateFailure,
    #[fail(display = "State is currently unavailable")]
    StateUnavailable,
    #[fail(display = "Configuration failure: {}", _0)]
    ConfigigurationFailure(crate::error::ConfigurationFailure),
    #[fail(display = "Requested feature is not available")]
    FeatureUnavailable,
}

impl From<sync::PoisonError<sync::RwLockWriteGuard<'_, Arc<Instance>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockWriteGuard<Arc<Instance>>>) -> Self {
        KatalystError::StateUpdateFailure
    }
}

impl From<sync::PoisonError<sync::RwLockReadGuard<'_, Arc<Instance>>>> for KatalystError {
    fn from(_: sync::PoisonError<sync::RwLockReadGuard<Arc<Instance>>>) -> Self {
        KatalystError::StateUnavailable
    }
}

impl From<AddrParseError> for KatalystError {
    fn from(_: AddrParseError) -> Self {
        KatalystError::ConfigigurationFailure(crate::error::ConfigurationFailure::InvalidAddress(
            "network",
        ))
    }
}

impl From<crate::error::ConfigurationFailure> for KatalystError {
    fn from(c: crate::error::ConfigurationFailure) -> Self {
        KatalystError::ConfigigurationFailure(c)
    }
}
