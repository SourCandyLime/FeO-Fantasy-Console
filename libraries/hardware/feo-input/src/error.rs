#[derive(Debug)]
pub enum InputError {
    InitializationFailed,
    UnsupportedPlatform,
}

use std::fmt;

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::InitializationFailed => {
                write!(f, "input initialization failed")
            }
            InputError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
        }
    }
}

impl std::error::Error for InputError {}