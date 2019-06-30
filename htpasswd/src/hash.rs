use crate::HtpasswdError;
use HashedPassword::*;

/// This represents the hashed password field in an HTPASSWD file
#[derive(Debug, PartialEq)]
pub enum HashedPassword {
    Bcrypt(String),
    Sha(String),
    Md5(String),
    Unknown(String),
}

impl HashedPassword {
    /// Check the provided password against a hash.
    /// Note that currently only BCRYPT passwords are supported!
    /// There are a few reasons for this, but the most important is
    /// that SHA1 and MD5 are not considered to be secure storage
    /// algorithms.
    pub fn verify(&self, pw: &str) -> Result<(), HtpasswdError> {
        match self {
            Bcrypt(h) => {
                if bcrypt::verify(pw, h)? {
                    Ok(())
                } else {
                    Err(HtpasswdError::ValidationError("Invalid password".into()))
                }
            }
            Sha(_) => Err(HtpasswdError::ValidationError("SHA1 passwords unsupported".into())),
            Md5(_) => Err(HtpasswdError::ValidationError("MD5 passwords unsupported".into())),
            _ => Err(HtpasswdError::ValidationError("Unrecognized password hash format".into())),
        }
    }
}

impl From<bcrypt::BcryptError> for HtpasswdError {
    fn from(_: bcrypt::BcryptError) -> HtpasswdError {
        HtpasswdError::ValidationError("Invalid password".into())
    }
}
