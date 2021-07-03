extern crate firecore_game as game;
extern crate firecore_battle as battle;

pub use game::gui::{party::PartyGui, bag::BagGui};

use battle::{
    data::{BattleData, BattleType},
    player::{BattlePlayer, PlayerSettings, ai::BattlePlayerAi},
    Battle,
};

use game::{
    battle_glue::{BattleEntry, BattleTrainerEntry},
    deps::{UNKNOWN16, rhai::Engine},
    pokedex::{pokemon::instance::BorrowedPokemon, trainer::TrainerId, moves::usage::script::engine},
    storage::player::PlayerSave,
};

mod manager;
mod guiref;

pub use manager::*;
pub use guiref::*;

pub struct GameBattleWrapper {
    pub battle: Option<GameBattle>,
    pub engine: Engine,
}

pub struct GameBattle {
    pub battle: Battle<TrainerId>,
    pub trainer: Option<BattleTrainerEntry>,
}

impl GameBattleWrapper {
    pub fn new() -> Self {
        Self {
            battle: None,
            engine: engine(),
        }
    }

    pub fn battle(&mut self, player: BattlePlayer<TrainerId>, entry: BattleEntry) {
        self.battle =
            Some(GameBattle {
                battle: Battle::new(
                    BattleData {
                        type_: entry
                            .trainer
                            .as_ref()
                            .map(|trainer| {
                                if trainer.gym_badge.is_some() {
                                    BattleType::GymLeader
                                } else {
                                    BattleType::Trainer
                                }
                            })
                            .unwrap_or(BattleType::Wild),
                    },
                    player,
                    BattlePlayer::new(
                        entry.trainer.as_ref().map(|t| t.id).unwrap_or(
                            UNKNOWN16,
                        ),
                        entry
                            .party
                            .into_iter()
                            .map(|instance| BorrowedPokemon::Owned(instance))
                            .collect(),
                        entry.trainer_data,
						PlayerSettings {
							gains_exp: false,
							request_party: false,
						},
                        Box::new(BattlePlayerAi::new(
                            UNKNOWN16,
                        )),
                        entry.size,
                    ),
                ),
                trainer: entry.trainer,
            });
    }
}

impl GameBattle {
    pub fn update_data(&mut self, winner: &TrainerId, player: &mut PlayerSave) -> bool {
        let trainer = self.trainer.is_some();

        if &player.id == winner {
            if let Some(trainer) = self.trainer.take() {
                player.worth += trainer.worth as u32;
                if let Some(badge) = trainer.gym_badge {
                    player.world.badges.insert(badge);
                }
            }
        }

        trainer
    }
}
