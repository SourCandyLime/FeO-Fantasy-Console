pub mod command;

pub mod exit;
pub mod help;
pub mod info;

// Re-export the trait.
pub use command::Command;
pub use command::CommandResult;

// Re-export the command structs.
pub use exit::Exit;
pub use help::Help;
pub use info::Info;

/// Every command known to the FeO shell.
pub fn all() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(Help),
        Box::new(Info),
        Box::new(Exit),
    ]
}