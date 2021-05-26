use deps::tetra::Context;

use super::{BattleData, pokemon::{ActivePokemonArray, BattleParty}};

pub mod gui;
pub mod ai;

pub trait BattlePlayerAction {

    // fn name(&self) -> Cow<str>;

    fn moves(&mut self, ctx: &Context, delta: f32, data: &BattleData, active: &mut usize, user: &mut BattleParty, target: &ActivePokemonArray) -> bool;

    fn faint(&mut self, ctx: &Context, delta: f32, data: &BattleData, index: usize, user: &mut BattleParty) -> bool;

    fn draw(&self, ctx: &mut Context);

}

pub struct BattlePlayer {
    pub player: Box<dyn BattlePlayerAction>,
    pub party: BattleParty,
}