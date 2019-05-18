use super::GatewayError;
use crate::instance::Instance;
use std::net::AddrParseError;
use std::sync;
use std::sync::Arc;

impl From<syn::Error> for GatewayError {
    fn from(e: syn::Error) -> Self {
        GatewayError::ExpressionLexicalError(format!("{}", e))
    }
}

impl From<regex::Error> for GatewayError {
    fn from(r: regex::Error) -> Self {
        GatewayError::InvalidRegex(format!("{}", r))
    }
}

impl From<http::method::InvalidMethod> for GatewayError {
    fn from(m: http::method::InvalidMethod) -> Self {
        GatewayError::InvalidHttpMethod(m.to_string())
    }
}

impl From<serde_yaml::Error> for GatewayError {
    fn from(m: serde_yaml::Error) -> Self {
        GatewayError::ConfigNotParseable(m.to_string())
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(m: serde_json::Error) -> Self {
        GatewayError::ConfigNotParseable(m.to_string())
    }
}
impl From<std::io::Error> for GatewayError {
    fn from(m: std::io::Error) -> Self {
        GatewayError::ConfigNotParseable(m.to_string())
    }
}

impl From<sync::PoisonError<sync::RwLockWriteGuard<'_, Arc<Instance>>>> for GatewayError {
    fn from(_: sync::PoisonError<sync::RwLockWriteGuard<Arc<Instance>>>) -> Self {
        GatewayError::StateUpdateFailure
    }
}

impl From<sync::PoisonError<sync::RwLockReadGuard<'_, Arc<Instance>>>> for GatewayError {
    fn from(_: sync::PoisonError<sync::RwLockReadGuard<Arc<Instance>>>) -> Self {
        GatewayError::StateUnavailable
    }
}

impl From<&'static str> for GatewayError {
    fn from(_: &'static str) -> Self {
        GatewayError::InternalServerError
    }
}

impl From<http::uri::InvalidUri> for GatewayError {
    fn from(_: http::uri::InvalidUri) -> Self {
        GatewayError::InternalServerError
    }
}

impl From<AddrParseError> for GatewayError {
    fn from(_: AddrParseError) -> Self {
        GatewayError::InvalidAddress("network")
    }
}
