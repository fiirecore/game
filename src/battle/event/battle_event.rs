use std::any::Any;

#[deprecated = "unused"]
pub trait BattleEvent {

    fn run(&mut self);

    fn get<E>(&self) -> E where E: Any;

}