use feo_debug::*;
use super::InputError;

pub struct Controller {
    id: u8
}

impl Controller {
    pub fn new(id: u8) -> Result<Self, InputError> {
        let controller = Self {
            id
        };
        log_boot(Status::Ok, format!("CT{}", id).as_str(), "Controller initialized");
        Ok(controller)
    }
}