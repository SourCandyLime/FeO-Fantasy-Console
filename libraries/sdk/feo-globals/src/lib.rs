pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 360;

pub const COLOR_BITS_RED: u8 = 5;
pub const COLOR_BITS_GREEN: u8 = 6;
pub const COLOR_BITS_BLUE: u8 = 5;

pub const FILE_EXTENSION: &str = ".feo";
pub const PLUG_EXTENSION: &str = ".plug";

pub const RAM_SIZE: usize = 4 * 1024 * 1024 * 1024;
pub const RAM_PAGE_SIZE: usize = 4096;
pub const RAM_PAGE_COUNT: usize = RAM_SIZE / RAM_PAGE_SIZE;
