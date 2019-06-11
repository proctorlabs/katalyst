/*!
Katalyst error types
*/

mod from;
mod result;

use http::StatusCode;
use std::error::Error;

pub use result::*;

type Source = Option<Box<SourceError + 'static>>;
pub trait SourceError: Error + std::fmt::Display + Send {}
impl<T: Error + Send + std::fmt::Display> SourceError for T {}

fn add_source(source: &Source) -> String {
    if let Some(s) = source {
        format!("\nCaused by: {}", s)
    } else {
        "".into()
    }
}

#[derive(Debug, Display)]
pub enum GatewayError {
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
        status: StatusCode,
        message: String,
        source: Source,
        module_path: &'static str,
        line: u32,
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    ConfigurationFailure {
        message: String,
        source: Source,
        module_path: &'static str,
        line: u32,
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Critical { message: String, source: Source, module_path: &'static str, line: u32, col: u32 },
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
        name: String,
        message: String,
        source: Source,
        module_path: &'static str,
        line: u32,
        col: u32,
    },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    IoError { message: String, source: Source, module_path: &'static str, line: u32, col: u32 },
    #[display(fmt = "[{} -> {}:{}] {}{}", module_path, line, col, message, "add_source(source)")]
    Other { message: String, source: Source, module_path: &'static str, line: u32, col: u32 },
    #[display(fmt = "Request finished early")]
    Done,
}

impl Error for GatewayError {}

impl GatewayError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            GatewayError::RequestFailed { status, .. } => status,
            GatewayError::Done => StatusCode::OK,
            GatewayError::IoError { .. } => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
