use super::Command;
use super::CommandResult;

pub struct Info;

impl Command for Info {
    fn name(&self) -> &'static str {
        "info"
    }

    fn help(&self) -> &'static str {
        "Displays information about the FeO Fantasy Console."
    }

    fn execute(&self, _args: &[&str]) -> CommandResult {
        println!("\nFeO Fantasy Console v0.0.1\n");
        println!("Hardware Specification:");
        println!("  Memory      : 4096 MB");
        println!("  Resolution  : 640 x 360");
        println!("  Color       : RGB 4-5-4");
        println!("  File Format : .feo");
        println!("  Plug Format : .plug\n");
        CommandResult::Continue
    }
}