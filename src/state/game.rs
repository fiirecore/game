use std::sync::atomic::Ordering::Relaxed;

use firecore_game::state::GameStateAction;
use game::{
	storage::{get, get_mut, DIRTY, save, player::PlayerSaves},
	macroquad::prelude::{info, warn, is_key_down, KeyCode},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	battle::BattleData,
};

use world::map::manager::WorldManager;
use battle::manager::BattleManager;

use super::State;
use super::States;

pub struct GameState {

	action: Option<GameStateAction>,
	
	world: WorldManager,
	battle: BattleManager,

	party_gui: PartyGui,
	bag_gui: BagGui,
	
	battle_data: Option<BattleData>,

	battling: bool,

}

impl GameState {

	pub async fn load(&mut self) {
		match postcard::from_bytes(include_bytes!("../../build/data/world.bin")) {
			Ok(world) => {
				self.world.load(world);
			}
			Err(err) => {
				panic!("Could not load world file with error {}", err);
			}
		}
	}

	pub fn data_dirty(&mut self, player_data: &mut PlayerSaves) {
		self.save_data(player_data);
		DIRTY.store(false, Relaxed);
	}

    pub fn save_data(&mut self, player_data: &mut PlayerSaves) {
        self.world.save_data(player_data.get_mut());
		info!("Saving player data!");
		if let Err(err) = save(player_data) {
			warn!("Could not save player data with error: {}", err);
		}
    }
	
}

impl State for GameState {

	fn new() -> Self {
		Self {

			action: None,

			world: WorldManager::new(),
			battle: BattleManager::new(),
			party_gui: PartyGui::new(),
			bag_gui: BagGui::new(),

			battle_data: None,

			battling: false,
		}
	}

	fn on_start(&mut self) {
		self.world.load_with_data();
		self.world.on_start(&mut self.battle_data);
	}
	
	fn update(&mut self, delta: f32) -> Option<States> {

		// Speed game up if spacebar is held down

		let delta = delta *  if is_key_down(KeyCode::Space) {
			4.0
		} else {
			1.0
		};

		if DIRTY.load(Relaxed) {
			if let Some(mut saves) = get_mut::<PlayerSaves>() {
				self.data_dirty(&mut saves);
			}	
		}

		if !self.battling {

			self.world.update(delta, &mut self.battle_data);

			if self.battle_data.is_some() {
				if let Some(player_saves) = get::<PlayerSaves>() {
					if self.battle.battle(&player_saves.get().party, self.battle_data.take().unwrap()) {
						self.battling = true;
					}
				}
			}

		} else {

			self.battle.update(delta, &mut self.party_gui, &mut self.bag_gui);
			
			if self.battle.is_finished() {
				if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
					let save = player_saves.get_mut();
					if let Some((winner, trainer)) = self.battle.update_data(save) {
						world::battle::update_world(&mut self.world, save, winner, trainer);
					}
				}				
				self.battling = false;
				self.world.map_start(true);
			}

		}

		self.bag_gui.update(&mut self.party_gui);
		self.party_gui.update(delta);

		self.action.take().map(|action| match action {
			GameStateAction::ExitToMenu => States::Menu,
		})

	}
	
	fn render(&self) {
		if self.party_gui.is_alive() {
			self.party_gui.render();
		} else if self.bag_gui.is_alive() {
			self.bag_gui.render();
		} else if !self.battling {
			self.world.render();
		} else {
			if self.battle.world_active() {
				self.world.render();
			}
			self.battle.render();
		}
	}
	
	fn input(&mut self, delta: f32) {
		if self.bag_gui.is_alive() {
			self.bag_gui.input(&mut self.party_gui);
		} else if self.party_gui.is_alive() {
			self.party_gui.input();
		} else if !self.battling {
			self.world.input(delta, &mut self.battle_data, &mut self.party_gui, &mut self.bag_gui, &mut self.action);
		} else {
			self.battle.input();
		}
	}

	fn quit(&mut self) {
		if let Some(mut player_data) = get_mut::<PlayerSaves>() {
			self.save_data(&mut player_data);
		}
	}

}