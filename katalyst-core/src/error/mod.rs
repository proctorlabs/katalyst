/*!
This module space contains the enum `GatewayError` which is the error type produced by
the internal KatalystCore libraries.
*/

mod from;
mod result;

use http::StatusCode;
use std::error::Error;

pub use result::*;

type Source = Option<Box<SourceError + 'static>>;
#[doc(hidden)]
pub trait SourceError: Error + std::fmt::Display + Send {}
impl<T: Error + Send + std::fmt::Display> SourceError for T {}

fn add_source(source: &Source) -> String {
    if let Some(s) = source {
        format!("\nCaused by: {}", s)
    } else {
        "".into()
    }
}

/// All KatalystCore library methods will return a variant of GatewayError
#[derive(Debug, Display)]
pub enum GatewayError {
    /// This is the primary type that is returned when there is some error that occurs
    /// while processing a request.
    #[display(
        fmt = "[{} -> {}:{}] <{}> {}{}",
        module_path,
        line,
        col,
        status,
        message,
        "add_source(source)"
    )]
    RequestFailed {
        #[doc(hidden)]
        status: StatusCode,
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },
    /// Error that occurs when there is a configuration failure
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    ConfigurationFailure {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },

    /// Catastrophic and fatal errors
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Critical {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },

    /// A dependency that was expected is not available
    #[display(
        fmt = "[{} -> {}:{}] Component {}: {}{}",
        module_path,
        line,
        col,
        name,
        message,
        "add_source(source)"
    )]
    RequiredComponent {
        #[doc(hidden)]
        name: String,
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },

    /// An IO Error, check the source error for more detail
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    IoError {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },

    /// Other uncategorized/general error
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Other {
        #[doc(hidden)]
        message: String,
        #[doc(hidden)]
        source: Source,
        #[doc(hidden)]
        module_path: &'static str,
        #[doc(hidden)]
        line: u32,
        #[doc(hidden)]
        col: u32,
    },

    /// Used in some circumstance to return from the request pipeline early
    #[display(fmt = "Request finished early")]
    Done,
}

impl Error for GatewayError {}

impl GatewayError {
    pub(crate) fn status_code(&self) -> StatusCode {
        match *self {
            GatewayError::RequestFailed { status, .. } => status,
            GatewayError::Done => StatusCode::OK,
            GatewayError::IoError { .. } => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
