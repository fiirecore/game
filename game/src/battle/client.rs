use deps::tetra::Context;

use super::{BattleData, pokemon::{BattleMove, BattlePartyPlayerView, BattlePartyView}};

pub mod gui;
pub mod ai;

pub trait BattleClient {

    // fn name(&self) -> Cow<str>;

    fn begin(&mut self, data: &BattleData);

    fn start_moves(&mut self, user: BattlePartyPlayerView, targets: BattlePartyView);

    fn wait_moves(&mut self) -> Option<Vec<BattleMove>>;

    fn start_faint(&mut self, active: usize);

    fn wait_faint(&mut self) -> Option<usize>;

    fn draw(&self, ctx: &mut Context);

}