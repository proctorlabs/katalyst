/*!
Katalyst error types
*/

mod from;
use http::StatusCode;

#[derive(Debug, Display)]
pub enum GatewayError {
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
    #[display(fmt = "{}", _1)]
    Other(StatusCode, &'static str),
}

impl std::error::Error for GatewayError {}

pub type Result<T> = std::result::Result<T, GatewayError>;

impl GatewayError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            GatewayError::NotFound => StatusCode::NOT_FOUND,
            GatewayError::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            GatewayError::Forbidden => StatusCode::FORBIDDEN,
            GatewayError::Unauthorized => StatusCode::UNAUTHORIZED,
            GatewayError::Other(code, _) => code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
