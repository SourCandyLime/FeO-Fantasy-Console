use super::SystemError;
use feo_debug::*;
use feo_memory::Memory;
use feo_clock::Clock;
use feo_input::Input;
use feo_audio::Audio;
use feo_physics::Physics;

pub struct System {
    memory: Memory,
    clock: Clock,
    input: Input,
    audio: Audio,
    physics: Physics,
}

impl System {
    pub fn init() -> Result<System, SystemError> {
        let memory = Memory::new().map_err(|e| SystemError::InitializationFailed)?;
        let clock = Clock::new().map_err(|e| SystemError::InitializationFailed)?;
        let input = Input::new().map_err(|e| SystemError::InitializationFailed)?;
        let audio = Audio::new().map_err(|e| SystemError::InitializationFailed)?;
        let physics = Physics::new().map_err(|e| SystemError::InitializationFailed)?;
        let system = Self {
            memory,
            clock,
            input,
            audio,
            physics,
        };

        log_boot(Status::Ok, "SYS", "System services initialized");
        
        Ok(system)
    }
}