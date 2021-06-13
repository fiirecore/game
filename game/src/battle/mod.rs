pub use firecore_battle::*;

use data::*;

use crate::{
	deps::rhai::Engine,
	battle::pokemon::BattlePlayer,
	pokedex::{
		pokemon::instance::BorrowedPokemon,
		moves::target::PlayerId,
	},
	storage::player::PlayerSave,
	battle_glue::{BattleEntry, BattleTrainerEntry},
};

use ai::BattlePlayerAi;

pub mod client_state;
pub mod manager;
pub mod ui;

pub mod ai;
pub mod gui;

pub struct GameBattle {
	pub battle: Battle,
	pub trainer: Option<BattleTrainerEntry>,
}

impl GameBattle {

	pub fn new(engine: Engine) -> Self {
		Self {
			battle: Battle::new(engine),
			trainer: None,
		}
	}
	
	pub fn battle(&mut self, player: BattlePlayer, entry: BattleEntry) {
		self.battle.battle(BattleHost::new(
			BattleData {
				type_: entry.trainer.as_ref().map(|trainer| if trainer.gym_badge.is_some() { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild),
			},
			player, 
			BattlePlayer::new(entry.trainer.as_ref().map(|t| t.id).unwrap_or("unknown".parse().unwrap()), entry.trainer.as_ref().map(|t| t.name.as_str()).unwrap_or("Wild"), entry.party.into_iter().map(|instance| BorrowedPokemon::Owned(instance)).collect(), Box::new(BattlePlayerAi::default()), entry.size)
		));
		self.trainer = entry.trainer;
	}

	pub fn update_data(&mut self, player: &mut PlayerSave) -> Option<(PlayerId, bool)> {

		let trainer = self.trainer.is_some();

		let winner = self.battle.state().map(|s| match s {
			state::BattleState::End(w) => Some(*w),
			_ => None,
		}).flatten();

		if let Some(winner) = &winner {
			if &player.id == winner {
				if let Some(trainer) = self.trainer.take() {
					player.worth += trainer.worth as u32;
					if let Some(badge) = trainer.gym_badge {
						player.world.badges.insert(badge);
					}
				}
			}
		}

		winner.map(|winner| (winner, trainer))
		
	}

}