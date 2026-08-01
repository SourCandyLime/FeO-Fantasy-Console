use feo_debug::*;
use super::PhysicsError;

pub struct Physics {
    pub gravity: f32,
}

impl Physics {
    pub fn new() -> Result<Self, PhysicsError> {
        let physics = Self {
            gravity: 9.81,
        };
        log_boot(Status::Ok, "PHY", "Physics services initialized");
        Ok(physics)
    }
}