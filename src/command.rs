#[derive(Debug, Clone)]
pub struct CommandResult<'a> {
    pub command: &'a str,
    pub args: std::str::SplitAsciiWhitespace<'a>,
}

pub trait CommandProcessor {

    fn process(&mut self, command: CommandResult);

}

impl<'a> core::fmt::Display for CommandResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.command, self.args)
    }
}