#[derive(Debug)]
pub enum AudioError {
    InitializationFailed,
    UnsupportedPlatform,
}

use std::fmt;

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::InitializationFailed => {
                write!(f, "audio initialization failed")
            }
            AudioError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
        }
    }
}

impl std::error::Error for AudioError {}