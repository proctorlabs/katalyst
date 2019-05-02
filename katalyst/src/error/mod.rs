/*!
Katalyst error types
*/

mod config;
mod internal;
mod request;

pub use config::ConfigurationFailure;
pub use internal::KatalystError;
pub use request::RequestFailure;
