use super::pokemon::ActivePokemonArray;

pub mod gui;
pub mod ai;

pub trait BattlePlayer {

    // fn name(&self) -> Cow<str>;

    fn moves(&mut self, active: &mut ActivePokemonArray, target: &ActivePokemonArray) -> bool;

    fn faint(&mut self, user: &mut ActivePokemonArray) -> bool;

}