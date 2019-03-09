use super::KatalystError;
use http::StatusCode;

impl KatalystError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            KatalystError::NotFound => StatusCode::NOT_FOUND,
            KatalystError::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
