#[derive(Debug)]
pub enum MemoryError {
    InitializationFailed,
    UnsupportedPlatform,
}


use std::fmt;

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InitializationFailed => {
                write!(f, "memory initialization failed")
            }
            MemoryError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
        }
    }
}

impl std::error::Error for MemoryError {}