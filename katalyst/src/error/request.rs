use failure::Context;
use http::StatusCode;

/// KatalystError
#[derive(Debug, Fail)]
pub enum RequestFailure {
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "Gateway Timeout")]
    GatewayTimeout,
    #[fail(display = "Forbidden")]
    Forbidden,
    #[fail(display = "Unauthorized")]
    Unauthorized,
    #[fail(display = "Internal Server Error")]
    Internal,
    #[fail(display = "{}", _1)]
    Other(StatusCode, &'static str),
}

impl RequestFailure {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            RequestFailure::NotFound => StatusCode::NOT_FOUND,
            RequestFailure::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            RequestFailure::Forbidden => StatusCode::FORBIDDEN,
            RequestFailure::Unauthorized => StatusCode::UNAUTHORIZED,
            RequestFailure::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            RequestFailure::Other(code, _) => code,
        }
    }
}

impl From<&'static str> for RequestFailure {
    fn from(_: &'static str) -> RequestFailure {
        RequestFailure::Internal
    }
}

// Allows adding more context via a String
impl From<Context<String>> for RequestFailure {
    fn from(_: Context<String>) -> RequestFailure {
        RequestFailure::Internal
    }
}

// Allows adding more context via a &str
impl From<Context<&'static str>> for RequestFailure {
    fn from(_: Context<&'static str>) -> RequestFailure {
        RequestFailure::Internal
    }
}
