use super::Command;
use super::CommandResult;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn help(&self) -> &'static str {
        "Exits the FeO Runtime."
    }

    fn execute(&self, _args: &[&str]) -> CommandResult {
        println!("Shutting down FeO...");
        CommandResult::Exit
    }
}