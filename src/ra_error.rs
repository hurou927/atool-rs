use std::{error::Error, fmt};

#[derive(Debug)]
pub enum RaError {
    UnSupportedFormat { path: String },
    OtherPathError { reason: String },
}

impl fmt::Display for RaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RaError::*;
        match self {
            UnSupportedFormat { path } => write!(f, "UnSupportedFormat: {}", path),
            OtherPathError { reason } => write!(f, "OtherPathError: {}", reason),
        }
    }
}

impl Error for RaError {}
