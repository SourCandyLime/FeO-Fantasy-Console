pub struct Console {
    commands: Vec<Box<dyn Command>>
}

impl Console {
    fn new() -> Self {
        let commands = crate::commands::all()
        Self {
            commands
        }
    }
}