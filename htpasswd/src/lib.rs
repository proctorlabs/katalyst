/*!
# HTPASSWD Parsing and validation

This library provides a parser and bcrypt validator for HTPASSWD files.

# Hash Support

Currently only bcrypt hashes are supported with the built in validation.

HTPASSWD supports a number of different password formats:

* Plain text: This simply stores the password in an easily readable format
* Unix crypt: Traditionally a unix only hash format
* SHA1: A fast hash format that is now considered to be insecure for password usage
* MD5: Apache uses a specialized MD5 format (APR1).
* Bcrypt: Supported out of the box in this library, this is the most secure algorithm for password storage

# Usage

```
# fn main() -> Result<(), htpasswd::HtpasswdError> {
let file_contents = "someuser:$2y$05$4hlGFwyiqrMxB4XS9.0nLeaKvU40nNmyv73UkrQmW8sUn9hdoa99."; // Password is 'test'
let result = htpasswd::parse(file_contents)?;
let user_hash = &result["someuser"];
user_hash.verify("test")
# }
```
*/

mod error;
mod hash;
mod parser;

#[cfg(test)]
mod test;

pub use error::HtpasswdError;
pub use hash::HashedPassword;
pub use parser::parse;
