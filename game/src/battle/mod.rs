pub use firecore_battle::*;

use crate::{
	battle::pokemon::BattleParty,
	pokedex::pokemon::instance::BorrowedPokemon,
	storage::player::{PlayerSave, PlayerId},
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
	
	pub fn new(player: BattleParty, entry: BattleEntry) -> Self {
		Self {
			battle: Battle::new(
				entry.trainer.as_ref().map(|trainer| if trainer.gym_badge.is_some() { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild), 
				player, 
				BattleParty::new("opponent".parse().unwrap(), "opponent", entry.party.into_iter().map(|instance| BorrowedPokemon::Owned(instance)).collect(), Box::new(BattlePlayerAi::default()), entry.size)
			),
			trainer: entry.trainer,
		}
	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(PlayerId, bool)> {

		let trainer = self.trainer.is_some();

		if let Some(winner) = self.battle.winner {
			if player.id == winner {
				if let Some(trainer) = self.trainer {
					player.worth += trainer.worth as u32;
					if let Some(badge) = trainer.gym_badge {
						player.world.badges.insert(badge);
					}
				}
			}
		}

		self.battle.winner.map(|winner| (winner, trainer))
		
	}

}