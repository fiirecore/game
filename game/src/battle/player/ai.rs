use deps::tetra::Context;
use pokedex::moves::target::MoveTargetInstance;

use crate::battle::{BattleData, pokemon::{ActivePokemonArray, BattleMove, BattleParty, PokemonOption}};

use super::BattlePlayerAction;

pub struct BattlePlayerAi;

impl BattlePlayerAction for BattlePlayerAi {

    fn moves(&mut self, ctx: &Context, delta: f32, data: &BattleData, active: &mut usize, user: &mut BattleParty, target: &ActivePokemonArray) -> bool { // note: does not use PP
        use crate::battle::BATTLE_RANDOM;
        // match self {
        //     BattleAi::Random => {
                for active in user.active.iter_mut() {
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
        //     }
        // }
        true
    }

    fn faint(&mut self, ctx: &Context, delta: f32, data: &BattleData, active_index: usize, user: &mut BattleParty) -> bool {
        let available: Vec<usize> = user.pokemon.iter()
            .enumerate()
            .filter(|(_, pokemon)| pokemon.as_ref().map(|instance| !instance.value().fainted()).unwrap_or(false))
            .map(|(index, _)| index)
            .collect();

        if !available.is_empty() {
            user.queue_replace(active_index, available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())]);
        } else {
            user.remove_pokemon(active_index);
        }
        true
    }

    fn draw(&self, ctx: &mut Context) {}

}