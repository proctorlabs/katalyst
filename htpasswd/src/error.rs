use std::fmt;
use HtpasswdError::*;

/// Represents possible errors in HTPASSWD operations
#[derive(Debug)]
pub enum HtpasswdError {
    /// Returned with parse failures
    ParseError(String),
    /// Returned when a password doesn't match
    ValidationError(String),
}

impl fmt::Display for HtpasswdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError(s) => write!(f, "Unable to parse htpasswd file. {}", s),
            ValidationError(s) => write!(f, "{}", s),
        }
    }
}
