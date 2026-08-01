mod audio;
mod error;

pub use audio::Audio;
pub use error::AudioError;

pub fn init() -> Result<Audio, AudioError> {
    Audio::new().map_err(|e| AudioError::InitializationFailed)
}