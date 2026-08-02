pub struct Console {
    commands: Commands
}

impl Console {
    fn new() -> Self {
        let commands = Commands::new()
        Self {
            commands
        }
    }
}