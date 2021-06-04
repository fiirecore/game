use pokedex::moves::target::MoveTargetInstance;

use crate::battle::{BattleData, pokemon::{BattleClientAction, BattleClientMove, BattleMove, BattlePartyKnown, BattlePartyTrait, BattlePartyUnknown, PokemonUnknown}};

use super::BattleClient;

#[derive(Default)]
pub struct BattlePlayerAi {

    user: BattlePartyKnown,
    targets: BattlePartyUnknown,

    moves: Option<Vec<BattleMove>>,
    faint: deps::hash::HashMap<usize, usize>,

}

impl BattleClient for BattlePlayerAi {

    fn begin(&mut self, _data: &BattleData, user: BattlePartyKnown, targets: BattlePartyUnknown) {
        self.user = user;
        self.targets = targets;
    }

    fn start_select(&mut self) { // note: does not use PP
        use crate::battle::BATTLE_RANDOM;
        self.moves = Some(
            self.user.active.iter().flat_map(|index| (*index).map(|index| &self.user.pokemon[index])).map(|pokemon| {
                    // crashes when moves run out
                    let moves: Vec<usize> = pokemon.moves.iter().enumerate().filter(|(_, instance)| instance.pp != 0).map(|(index, _)| index).collect();
                    
                    BattleMove::Move(
                        moves[BATTLE_RANDOM.gen_range(0, moves.len())], 
                        MoveTargetInstance::opponent(BATTLE_RANDOM.gen_range(0, self.targets.active.len()))
                    )
            }).collect()
        );
    }

    fn wait_select(&mut self) -> Option<Vec<BattleMove>> {
        self.moves.take()
    }

    fn start_moves(&mut self, queue: Vec<crate::battle::pokemon::BattleClientActionInstance>) {
        for instance in queue {
            match instance.action {
                BattleClientAction::Move(_, actions) => {
                    for (_, moves) in actions {
                        for moves in moves {
                            match moves {
                                BattleClientMove::Faint(fainter) => {
                                    if fainter.team == pokedex::moves::target::Team::Player {
                                        if self.user.any_inactive() {

                                            let available: Vec<usize> = self.user.pokemon.iter()
                                                .enumerate()
                                                .filter(|(_, pokemon)| !pokemon.fainted())
                                                .map(|(index, _)| index)
                                                .collect();
                                    
                                            self.faint.insert(fainter.index, available[crate::battle::BATTLE_RANDOM.gen_range(0, available.len())]);
                                        }
                                    }
                                },
                                _ => (),
                            }
                        }
                    }
                },
                _ => (),
            }
        }
    }

    fn wait_faint(&mut self, active: usize) -> Option<usize> {
        self.faint.remove(&active)
    }

    fn opponent_faint_replace(&mut self, active: usize, new: Option<usize>, unknown: Option<PokemonUnknown>) {
        if let (Some(new), Some(unknown)) = (new, unknown) {
            self.targets.add(new, unknown);
        }
        self.targets.replace(active, new)
    }

    fn wait_finish_turn(&mut self) -> bool {
        true
    }

}