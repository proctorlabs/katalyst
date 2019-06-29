use crate::HtpasswdError;
use HashedPassword::*;

#[derive(Debug, PartialEq)]
pub enum HashedPassword {
    Bcrypt(String),
    Sha(String),
    Md5(String),
    Unknown(String),
}

impl HashedPassword {
    pub fn verify(&self, pw: &str) -> Result<(), HtpasswdError> {
        match self {
            Bcrypt(h) => {
                if bcrypt::verify(pw, h)? {
                    Ok(())
                } else {
                    Err(HtpasswdError::ValidationError("Invalid password".into()))
                }
            }
            _ => Err(HtpasswdError::ValidationError("Unsupported password format!".into())),
        }
    }
}

impl From<bcrypt::BcryptError> for HtpasswdError {
    fn from(_: bcrypt::BcryptError) -> HtpasswdError {
        HtpasswdError::ValidationError("Invalid password".into())
    }
}
