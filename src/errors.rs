use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ProxyError;

impl Error for ProxyError {}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid proxy")
    }
}
