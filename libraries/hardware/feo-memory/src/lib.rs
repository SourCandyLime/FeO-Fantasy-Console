mod memory;
mod ram;
mod error;

pub use memory::Memory;
pub use ram::Ram;
pub use error::MemoryError;

use feo_debug::*;

pub fn init() -> Result<Memory, MemoryError> {
    let memory = Memory::new()?;
    log_boot(Status::Ok, "MEM", "Memory services initialized");
    Ok(memory)
}