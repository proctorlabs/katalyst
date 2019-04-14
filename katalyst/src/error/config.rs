#[derive(Debug, Fail)]
pub enum ConfigurationFailure {
    #[fail(display = "Expected element {} that was missing!", _0)]
    ElementExpected(&'static str),
    #[fail(display = "Failed to parse expression: {}", _0)]
    ExpressionLexicalError(String),
    #[fail(display = "The specified object {} was not found!", _0)]
    ExpressionItemNotFound(String),
    #[fail(display = "Arguments are invalid: {}", _0)]
    InvalidExpressionArgs(&'static str),
    #[fail(display = "Provided regex is invalid: {}", _0)]
    InvalidRegex(String),
    #[fail(display = "Requested a resource type not available in this instance")]
    InvalidResource,
    #[fail(display = "Supplied address '{}' is invalid", _0)]
    InvalidAddress(&'static str),
}

impl From<syn::Error> for ConfigurationFailure {
    fn from(e: syn::Error) -> ConfigurationFailure {
        ConfigurationFailure::ExpressionLexicalError(format!("{}", e))
    }
}

impl From<regex::Error> for ConfigurationFailure {
    fn from(r: regex::Error) -> Self {
        ConfigurationFailure::InvalidRegex(format!("{}", r))
    }
}

impl From<crate::error::KatalystError> for ConfigurationFailure {
    fn from(_: crate::error::KatalystError) -> Self {
        ConfigurationFailure::InvalidResource
    }
}
