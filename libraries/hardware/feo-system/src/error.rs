#[derive(Debug)]
pub enum SystemError {
    UnsupportedPlatform,
    InitializationFailed,
}


use std::fmt;

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
            SystemError::InitializationFailed => {
                write!(f, "system initialization failed")
            }
        }
    }
}

impl std::error::Error for SystemError {}