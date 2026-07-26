pub trait Command {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn execute(&self, args: &[&str]) -> CommandResult;
}

pub enum CommandResult {
    Continue,
    Exit,
}