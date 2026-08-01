mod clock;
mod error;

pub use clock::Clock;
pub use error::ClockError;

pub fn new() -> Result<Clock, ClockError>{
    Clock::new().map_err(|e| ClockError::InitializationFailed)
}