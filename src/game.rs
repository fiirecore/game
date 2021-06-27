pub enum GameStateAction {
    ExitToMenu,
}

#[derive(Debug, Clone)]
pub struct CommandResult<'a> {
    pub command: &'a str,
    pub args: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> core::fmt::Display for CommandResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.command, self.args)
    }
}