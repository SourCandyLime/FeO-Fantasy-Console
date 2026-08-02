
pub struct Runtime {
    system: System,
    gfx: Graphics,
    console: Console,
    sdk: FeO
}

impl Runtime {
    fn init() -> Self {
        print("Runtime Init")
    }
}