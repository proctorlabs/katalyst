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
            KatalystError::NotFound => write!(f, "Not found!"),
        }
    }
}
