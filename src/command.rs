use std::{cell::RefCell, rc::Rc};

pub type CommandProcessor = Rc<CommandProcessors>;

#[derive(Default)]
pub struct CommandProcessors {
    pub commands: RefCell<Vec<String>>,
    pub errors: RefCell<Vec<&'static str>>,
}
