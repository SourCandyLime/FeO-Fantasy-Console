use super::{Controller, InputError};
use feo_debug::*;

pub struct Input {
    controllers: Vec<Controller>,
}

impl Input {
    pub fn new() -> Result<Self, InputError> {
        let input = Self {
            controllers: vec![Controller::new(1).map_err(|e| InputError::InitializationFailed)?]
        };
        log_boot(Status::Ok, "INP", "Input services initialized");
        Ok(input)
    }
}