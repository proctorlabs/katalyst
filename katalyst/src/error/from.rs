use super::GatewayError;
use std::net::AddrParseError;

impl From<std::io::Error> for GatewayError {
    fn from(err: std::io::Error) -> Self {
        err!(IoError, "An IO Error occurred", err)
    }
}

impl From<regex::Error> for GatewayError {
    fn from(err: regex::Error) -> Self {
        err!(ConfigurationFailure, "Could not compile regex", err)
    }
}

impl From<http::method::InvalidMethod> for GatewayError {
    fn from(err: http::method::InvalidMethod) -> Self {
        err!(ConfigurationFailure, "Invalid HTTP method", err)
    }
}

impl From<serde_yaml::Error> for GatewayError {
    fn from(err: serde_yaml::Error) -> Self {
        err!(ConfigurationFailure, "Configuration parse error", err)
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(err: serde_json::Error) -> Self {
        err!(ConfigurationFailure, "Configuration parse error", err)
    }
}

impl From<pest::error::Error<crate::expression::compiler::nodes::Rule>> for GatewayError {
    fn from(err: pest::error::Error<crate::expression::compiler::nodes::Rule>) -> Self {
        err!(ConfigurationFailure, "Invalid expression", err)
    }
}

impl From<std::num::ParseIntError> for GatewayError {
    fn from(err: std::num::ParseIntError) -> Self {
        err!(ConfigurationFailure, "Invalid number format", err)
    }
}

impl From<http::uri::InvalidUri> for GatewayError {
    fn from(e: http::uri::InvalidUri) -> Self {
        err!(ConfigurationFailure, "Unable to parse URI", e)
    }
}

impl From<AddrParseError> for GatewayError {
    fn from(e: AddrParseError) -> Self {
        err!(ConfigurationFailure, "Unable to parse network address", e)
    }
}

impl From<&'static str> for GatewayError {
    fn from(err: &'static str) -> Self {
        err!(Other, err)
    }
}

impl From<String> for GatewayError {
    fn from(err: String) -> Self {
        err!(Other, err)
    }
}
