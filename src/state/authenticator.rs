use crate::authentication::KatalystAuthenticator;
use std::sync::Arc;

#[derive(Debug)]
pub struct Authenticator {
    pub authenticator: Arc<KatalystAuthenticator>,
}
