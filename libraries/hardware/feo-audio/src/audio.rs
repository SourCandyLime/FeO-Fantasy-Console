use feo_debug::*;
use super::AudioError;

pub struct Audio {
    pub volume: f32,
}

impl Audio {
    pub fn new() -> Result<Self, AudioError> {
        let audio = Self {
            volume: 1.0,
        };
        log_boot(Status::Ok, "AUD", "Audio services initialized");
        Ok(audio)
    }
}