use std::sync::atomic::Ordering::Relaxed;

use firecore_game::state::GameStateAction;
use game::{
	storage::{PLAYER_SAVES, save, data_mut, player::{SHOULD_SAVE, PlayerSaves}},
	macroquad::prelude::{info, warn, is_key_down, KeyCode},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	battle_glue::BattleEntry,
};

use game::world::map::manager::WorldManager;
use game::battle::manager::BattleManager;

use super::State;
use super::States;

pub struct GameState {

	action: Option<GameStateAction>,

	state: GameStates,
	
	world: WorldManager,
	battle: BattleManager,

	party: PartyGui,
	bag: BagGui,
	
	battle_entry: Option<BattleEntry>,

}

pub enum GameStates {
	World,
	Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

impl GameState {

	pub async fn load(&mut self) {
		match game::deps::ser::deserialize(include_bytes!("../../build/data/world.bin")) {
			Ok(world) => self.world.load(world),
			Err(err) => panic!("Could not load world file with error {}", err),
		}
	}

	pub fn data_dirty(&mut self, saves: &mut PlayerSaves) {
		self.save_data(saves);
		SHOULD_SAVE.store(false, Relaxed);
	}

    pub fn save_data(&mut self, saves: &mut PlayerSaves) {
        self.world.save_data(saves.get_mut());
		info!("Saving player data!");
		if let Err(err) = save(saves) {
			warn!("Could not save player data with error: {}", err);
		}
    }
	
}

impl State for GameState {

	fn new() -> Self {
		Self {

			action: None,

			state: GameStates::default(),
			
			world: WorldManager::new(),
			battle: BattleManager::new(),
			party: PartyGui::new(),
			bag: BagGui::new(),

			battle_entry: None,
		}
	}

	fn on_start(&mut self) {
		self.world.load_with_data();
		self.world.on_start(&mut self.battle_entry);
	}
	
	fn update(&mut self, delta: f32) -> Option<States> {

		// Speed game up if spacebar is held down

		let delta = delta *  if is_key_down(KeyCode::Space) {
			4.0
		} else {
			1.0
		};

		if SHOULD_SAVE.load(Relaxed) {
			if let Some(mut saves) = unsafe{PLAYER_SAVES.as_mut()} {
				self.data_dirty(&mut saves);
			}	
		}
		match self.state {
			GameStates::World => {
				self.world.update(delta, &mut self.battle_entry, &mut self.party, &mut self.bag, &mut self.action);
				if let Some(entry) = self.battle_entry.take() {
					if self.battle.battle(entry) {
						self.state = GameStates::Battle;
					}
				}
			}
			GameStates::Battle => {
				self.battle.update(delta, &mut self.party, &mut self.bag);
				if self.battle.finished {
					let save = data_mut();
					if let Some((winner, trainer)) = self.battle.update_data(save) {
						game::world::battle::update_world(&mut self.world, save, winner, trainer);
					}			
					self.state = GameStates::World;
					self.world.map_start(true);
				}
			}
		}

		self.action.take().map(|action| match action {
			GameStateAction::ExitToMenu => States::Menu,
		})

	}
	
	fn draw(&self) {
		if self.party.alive {
			self.party.render();
		} else if self.bag.alive {
			self.bag.render();
		} else {
			match self.state {
				GameStates::World => self.world.render(),
				GameStates::Battle => {
					if self.battle.world_active() {
						self.world.render();
					}
					self.battle.render();
				}
			}
		}
	}

	fn quit(&mut self) {
		if let Some(mut saves) = unsafe{PLAYER_SAVES.as_mut()} {
			self.save_data(&mut saves);
		}
	}

}