pub enum GameStateAction {
    ExitToMenu,
}

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub command: String,
    pub args: Vec<String>,
}

impl core::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.command, self.args)
    }
}

pub trait GameState {
    fn process(&mut self, command: CommandResult);
    fn draw(&self, ctx: &mut deps::tetra::Context);
}