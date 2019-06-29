#[derive(Debug)]
pub enum HtpasswdError {
    ParseError(String),
    ValidationError(String),
}
