use std::{error::Error, fmt};

#[derive(Debug)]
pub enum RaError {
    UnSupportedFormat { path: String },
}

impl fmt::Display for RaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RaError::*;
        match self {
            UnSupportedFormat { path } => write!(f, "UnSupportedFormat: {}", path),
        }
    }
}

impl Error for RaError {}
