mod input;
mod controller;
mod error;

pub use input::Input;
pub use controller::Controller;
pub use error::InputError;

pub fn new() -> Result<Input, InputError> {
    Input::new().map_err(|e| InputError::InitializationFailed)
}