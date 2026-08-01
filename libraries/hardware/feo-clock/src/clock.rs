use feo_debug::*;
use super::ClockError;

pub struct Clock {
    fps: u16
}

impl Clock {
    pub fn new() -> Result<Self, ClockError> {
        let clock = Self {
            fps: 60
        };
        log_boot(Status::Ok, "CLK", "Clock services initialized");
        Ok(clock)
    }
}