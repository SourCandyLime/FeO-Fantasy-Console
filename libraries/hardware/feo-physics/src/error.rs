#[derive(Debug)]
pub enum PhysicsError {
    InitializationFailed,
    UnsupportedPlatform,
}

use std::fmt;

impl fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicsError::InitializationFailed => {
                write!(f, "physics initialization failed")
            }
            PhysicsError::UnsupportedPlatform => {
                write!(f, "unsupported platform")
            }
        }
    }
}

impl std::error::Error for PhysicsError {}