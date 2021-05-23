use crate::pokedex::moves::target::MoveTargetInstance;

use super::{
    ActivePokemonArray,
    BattleMove,
    PokemonOption, 
};

pub enum BattleAi {
    Random,
}

impl BattleAi {

    // pub fn select(&self) -> bool {} // wait on this function until it gives output, maybe make into trait

    #[deprecated(note = "make this input handler for MoveState::Selecting, not just a move generator")]
    pub fn moves(&self, user: &mut ActivePokemonArray, target: &ActivePokemonArray) { // note: does not use PP
        use crate::battle::BATTLE_RANDOM;
        match self {
            BattleAi::Random => {
                for active in user.iter_mut() {
                    if let PokemonOption::Some(_, pokemon) = &active.pokemon {

                        // crashes when moves run out
                        let moves: Vec<usize> = pokemon.value().moves.iter().enumerate().filter(|(_, instance)| instance.pp != 0).map(|(index, _)| index).collect();
                        
                        active.queued_move = Some(
                            BattleMove::Move(
                                moves[BATTLE_RANDOM.gen_range(0, moves.len())], 
                                MoveTargetInstance::Opponent(
                                    BATTLE_RANDOM.gen_range(0, target.len())
                                )
                            )
                        );
                    }
                }
            }
        }
    }
}