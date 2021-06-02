use deps::tetra::Context;
use pokedex::moves::target::MoveTargetInstance;

use crate::battle::{BattleData, pokemon::{BattleMove, BattleParty, BattlePartyPlayerView, BattlePartyView}};

use super::BattleClient;

#[derive(Default)]
pub struct BattlePlayerAi {
    user: BattlePartyPlayerView,
    moves: Option<Vec<BattleMove>>,
    faint: Option<usize>,
}

impl BattleClient for BattlePlayerAi {

    fn begin(&mut self, data: &BattleData) {
        
    }

    fn start_moves(&mut self, user: BattlePartyPlayerView, target: BattlePartyView) { // note: does not use PP
        self.user = user;
        use crate::battle::BATTLE_RANDOM;
        self.moves = Some(
            self.user.active.iter().flat_map(|pkmn| pkmn.as_ref()).map(|pokemon| {
                    // crashes when moves run out
                    let moves: Vec<usize> = pokemon.moves.iter().enumerate().filter(|(_, instance)| instance.pp != 0).map(|(index, _)| index).collect();
                                    
                    BattleMove::Move(
                        moves[BATTLE_RANDOM.gen_range(0, moves.len())], 
                        MoveTargetInstance::Opponent(
                            BATTLE_RANDOM.gen_range(0, target.active.len())
                        )
                    )
            }).collect()
        );
    }

    fn wait_moves(&mut self) -> Option<Vec<BattleMove>> {
        self.moves.take()
    }

    fn start_faint(&mut self, active: usize) {
        if let Some(pokemon) = self.user.active[active].as_mut() {
            pokemon.current_hp = 0;
        }
        let available: Vec<usize> = self.user.pokemon.iter()
            .enumerate()
            .filter(|(_, pokemon)| pokemon.as_ref().map(|instance| !instance.value().fainted()).unwrap_or(false))
            .map(|(index, _)| index)
            .collect();

        self.faint = Some(available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())]);

    }

    fn wait_faint(&mut self) -> Option<usize> {
        self.faint.take()
    }

    fn draw(&self, ctx: &mut Context) {}

}