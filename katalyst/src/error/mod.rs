/*!
Katalyst error types
*/

mod from;
mod result;

use http::StatusCode;
use std::error::Error;

pub use result::*;

type Source = Box<Error + Send>;

#[derive(Debug, Display)]
pub enum GatewayError {
    #[display(fmt = "Configuration Failure: {}\nCaused by: {:?}", message, source)]
    ConfigurationFailure { message: String, source: Source },
    #[display(fmt = "Critical internal failure: {}\nCaused by: {:?}", message, source)]
    Critical { message: String, source: Source },
    #[display(fmt = "Unable to update internal state")]
    StateUpdateFailure,
    #[display(fmt = "State is currently unavailable")]
    StateUnavailable,
    #[display(fmt = "Requested feature is not available")]
    FeatureUnavailable,
    #[display(fmt = "Expected element {} that was missing!", _0)]
    ElementExpected(&'static str),
    #[display(fmt = "Failed to parse expression: {}", _0)]
    ExpressionLexicalError(String),
    #[display(fmt = "The specified object {} was not found!", _0)]
    ExpressionItemNotFound(String),
    #[display(fmt = "Arguments are invalid: {}", _0)]
    InvalidExpressionArgs(&'static str),
    #[display(fmt = "Provided regex is invalid: {}", _0)]
    InvalidRegex(String),
    #[display(fmt = "Requested a resource type not available in this instance")]
    InvalidResource,
    #[display(fmt = "Supplied address '{}' is invalid", _0)]
    InvalidAddress(&'static str),
    #[display(fmt = "Unknown HTTP method specified '{}'", _0)]
    InvalidHttpMethod(String),
    #[display(fmt = "Could not parse configuration file: {}", _0)]
    ConfigNotParseable(String),
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "Gateway Timeout")]
    GatewayTimeout,
    #[display(fmt = "Forbidden")]
    Forbidden,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Request finished early")]
    Done,
    #[display(fmt = "IO Error occurred")] //: {:?}", _0)]
    IoError(std::io::Error),
    #[display(fmt = "{}", _1)]
    RequestFailed(StatusCode, &'static str),
    #[display(fmt = "{}", _0)]
    Other(String),
}

impl Error for GatewayError {}

impl GatewayError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            GatewayError::NotFound => StatusCode::NOT_FOUND,
            GatewayError::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            GatewayError::Forbidden => StatusCode::FORBIDDEN,
            GatewayError::Unauthorized => StatusCode::UNAUTHORIZED,
            GatewayError::RequestFailed(code, _) => code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
