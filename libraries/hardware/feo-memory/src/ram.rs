use feo_debug::*;
use feo_globals::*;
use super::MemoryError;

pub struct Ram {
    pages: Vec<Option<Box<[u8; RAM_PAGE_SIZE]>>>,
}

impl Ram {
    pub fn new() -> Result<Self, MemoryError> {
        let ram =Self {
            pages: vec![None; RAM_PAGE_COUNT]
        };
        log_boot(Status::Ok, "RAM", "RAM services initialized");
        Ok(ram)
    }
}