use super::{BattleType, pokemon::{BattleClientActionInstance, BattleMove, view::{BattlePartyKnown, BattlePartyUnknown, PokemonUnknown}}};

pub trait BattleClient {

    // fn name(&self) -> Cow<str>;

    fn user(&mut self, battle_type: BattleType, user: BattlePartyKnown);

    fn opponents(&mut self, opponent: BattlePartyUnknown); // maybe can send multiple

    fn add_unknown(&mut self, index: usize, unknown: PokemonUnknown);


    fn start_select(&mut self);

    fn wait_select(&mut self) -> Option<Vec<BattleMove>>;

    fn start_moves(&mut self, queue: Vec<BattleClientActionInstance>);


    // #[deprecated]
    // fn start_faint(&mut self, active: usize);

    fn wait_faint(&mut self, active: usize) -> Option<usize>;

    fn opponent_faint_replace(&mut self, active: usize, new: Option<usize>);


    fn wait_finish_turn(&mut self) -> bool;

    fn should_forfeit(&mut self) -> bool;

}