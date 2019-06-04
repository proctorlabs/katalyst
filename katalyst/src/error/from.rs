use super::GatewayError::{self, *};
use std::net::AddrParseError;

impl From<std::io::Error> for GatewayError {
    fn from(err: std::io::Error) -> Self {
        GatewayError::IoError(err)
    }
}

impl From<regex::Error> for GatewayError {
    fn from(err: regex::Error) -> Self {
        ConfigurationFailure { message: "Could not compile regex!".into(), source: Box::new(err) }
    }
}

impl From<http::method::InvalidMethod> for GatewayError {
    fn from(err: http::method::InvalidMethod) -> Self {
        ConfigurationFailure { message: "Invalid HTTP method".into(), source: Box::new(err) }
    }
}

impl From<serde_yaml::Error> for GatewayError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigurationFailure { message: "Configuration parse error".into(), source: Box::new(err) }
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        ConfigurationFailure { message: "Configuration parse error".into(), source: Box::new(err) }
    }
}

impl From<pest::error::Error<crate::expression::compiler::nodes::Rule>> for GatewayError {
    fn from(err: pest::error::Error<crate::expression::compiler::nodes::Rule>) -> Self {
        ConfigurationFailure { message: "Invalid expression".into(), source: Box::new(err) }
    }
}

impl From<std::num::ParseIntError> for GatewayError {
    fn from(err: std::num::ParseIntError) -> Self {
        ConfigurationFailure { message: "Invalid number format".into(), source: Box::new(err) }
    }
}

impl From<http::uri::InvalidUri> for GatewayError {
    fn from(err: http::uri::InvalidUri) -> Self {
        ConfigurationFailure { message: "Unable to parse URI".into(), source: Box::new(err) }
    }
}

impl From<AddrParseError> for GatewayError {
    fn from(err: AddrParseError) -> Self {
        ConfigurationFailure {
            message: "Unable to parse network address".into(),
            source: Box::new(err),
        }
    }
}

impl From<&'static str> for GatewayError {
    fn from(err: &'static str) -> Self {
        Other(err.into())
    }
}

impl From<String> for GatewayError {
    fn from(err: String) -> Self {
        Other(err)
    }
}
