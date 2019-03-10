mod conversions;
mod status_codes;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum KatalystError {
    StateUpdateFailure,
    StateUnavailable,
    ConfigFailure,
    Unavailable,
    ConfigParseError,
    NotFound,
    GatewayTimeout,
    Forbidden,
    Unauthorized,
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
            KatalystError::NotFound => write!(f, "Not Found"),
            KatalystError::GatewayTimeout => write!(f, "Gateway Timeout"),
            KatalystError::Forbidden => write!(f, "Forbidden"),
            KatalystError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}
