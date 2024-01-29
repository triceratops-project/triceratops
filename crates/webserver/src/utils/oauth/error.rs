use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub struct OpenIdConfigError;

impl fmt::Display for OpenIdConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error reading OpenId Configuration from remote host")
    }
}

impl Context for OpenIdConfigError {}