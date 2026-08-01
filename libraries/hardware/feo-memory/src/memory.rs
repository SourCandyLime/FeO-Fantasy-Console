use super::{Ram, MemoryError};
use feo_debug::*;

pub struct Memory {
    ram: Ram,
}

impl Memory {
    pub fn new() -> Result<Self, MemoryError> {
        let ram = Ram::new().map_err(|e| MemoryError::InitializationFailed)?;
        Ok(Self { ram })
    }
}