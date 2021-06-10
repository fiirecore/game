pub enum GameStateAction {
    ExitToMenu,
}

#[derive(Debug, Clone)]
pub struct CommandResult<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> core::fmt::Display for CommandResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.command, self.args)
    }
}

pub trait GameState {
    fn process(&mut self, result: CommandResult);
    fn draw(&self, ctx: &mut deps::tetra::Context);
}