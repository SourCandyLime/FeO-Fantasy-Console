#[derive(Debug)]
pub enum ClockError {
    InitializationFailed,
    UnsupportedPlatform,
}

use std::fmt;

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClockError::InitializationFailed => {
                write!(f, "clock initialization failed")
            }
            ClockError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
        }
    }
}

impl std::error::Error for ClockError {}