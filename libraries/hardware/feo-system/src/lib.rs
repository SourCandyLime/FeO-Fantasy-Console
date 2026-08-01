mod system;
mod error;

pub use system::System;
pub use error::SystemError;

pub fn init() -> Result<System, SystemError> {
    System::init()
}