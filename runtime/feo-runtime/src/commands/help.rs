use super::Command;
use super::CommandResult;

pub struct Help;

impl Command for Help {
    fn name(&self) -> &'static str {
        "help"
    }

    fn help(&self) -> &'static str {
        "Lists all commands."
    }

    fn execute(&self, _args: &[&str]) -> CommandResult {
        println!("Not implemented.");
        CommandResult::Continue
    }
}