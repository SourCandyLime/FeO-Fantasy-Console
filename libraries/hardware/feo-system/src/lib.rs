

pub struct System {
    name: &'static str,
    version: &'static str,
}

impl System {
    pub fn new() -> Self {
        Self {
            name: "FeO Runtime",
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}


pub fn init() -> Result<System, &'static str> {
    let system = System::new();

    feo_debug::log_boot(
        feo_debug::Status::Ok,
        "SYS",
        "System services initialized",
    );

    Ok(system)
}

pub fn os() -> &'static str {
    std::env::consts::OS
}

pub fn arch() -> &'static str {
    std::env::consts::ARCH
}
