use super::{BattleData, pokemon::{BattleClientActionInstance, BattleMove, BattlePartyKnown, BattlePartyUnknown, PokemonUnknown}};

pub mod gui;
pub mod ai;

pub trait BattleClient {

    // fn name(&self) -> Cow<str>;

    fn begin(&mut self, data: &BattleData, user: BattlePartyKnown, targets: BattlePartyUnknown);


    fn start_select(&mut self);

    fn wait_select(&mut self) -> Option<Vec<BattleMove>>;

    fn start_moves(&mut self, queue: Vec<BattleClientActionInstance>);


    // #[deprecated]
    // fn start_faint(&mut self, active: usize);

    fn wait_faint(&mut self, active: usize) -> Option<usize>;

    fn opponent_faint_replace(&mut self, active: usize, new: Option<usize>, unknown: Option<PokemonUnknown>);


    fn wait_finish_turn(&mut self) -> bool;

}