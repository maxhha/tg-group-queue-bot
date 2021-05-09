use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NoBotUsernameError;

impl fmt::Display for NoBotUsernameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cant get bots username")
    }
}

impl Error for NoBotUsernameError {}
