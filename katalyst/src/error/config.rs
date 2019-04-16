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
    #[fail(display = "Unknown HTTP method specified '{}'", _0)]
    InvalidHttpMethod(String),
    #[fail(display = "Could not parse configuration file: {}", _0)]
    ConfigNotParseable(String),
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

impl From<http::method::InvalidMethod> for ConfigurationFailure {
    fn from(m: http::method::InvalidMethod) -> Self {
        ConfigurationFailure::InvalidHttpMethod(m.to_string())
    }
}

impl From<serde_yaml::Error> for ConfigurationFailure {
    fn from(m: serde_yaml::Error) -> Self {
        ConfigurationFailure::ConfigNotParseable(m.to_string())
    }
}

impl From<serde_json::Error> for ConfigurationFailure {
    fn from(m: serde_json::Error) -> Self {
        ConfigurationFailure::ConfigNotParseable(m.to_string())
    }
}
impl From<std::io::Error> for ConfigurationFailure {
    fn from(m: std::io::Error) -> Self {
        ConfigurationFailure::ConfigNotParseable(m.to_string())
    }
}
