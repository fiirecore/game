use battle::{Battle, data::{BattleType, BattleData}, pokemon::BattlePlayer};

use crate::{
	deps::rhai::Engine,
	pokedex::{
		pokemon::instance::BorrowedPokemon,
		trainer::TrainerId,
	},
	storage::player::PlayerSave,
	battle_glue::{BattleEntry, BattleTrainerEntry},
};

use clients::ai::BattlePlayerAi;

pub mod manager;
pub mod ui;

pub mod clients;

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
			engine: pokedex::moves::usage::script::engine(),
		}
	}

	pub fn battle(&mut self, player: BattlePlayer<TrainerId>, entry: BattleEntry) {
		self.battle = Some(GameBattle {
			battle: Battle::new(
				BattleData {
					type_: entry.trainer.as_ref().map(|trainer| if trainer.gym_badge.is_some() { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild),
				},
				player, 
				BattlePlayer::new(entry.trainer.as_ref().map(|t| t.id).unwrap_or(<pokedex::moves::Move as deps::borrow::Identifiable>::UNKNOWN), entry.trainer_data, entry.party.into_iter().map(|instance| BorrowedPokemon::Owned(instance)).collect(), Box::new(BattlePlayerAi::new(<pokedex::moves::Move as deps::borrow::Identifiable>::UNKNOWN)), entry.size)
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