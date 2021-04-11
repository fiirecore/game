use std::sync::atomic::Ordering::Relaxed;

use game::{
	data::{get, get_mut, DIRTY, save, player::PlayerSaves},
	input::{pressed, Control},
	scene::SceneState,
	macroquad::prelude::{info, warn, is_key_down, is_key_pressed, KeyCode},
	gui::party::PokemonPartyGui,
	battle::BattleData,
};

use world::map::manager::WorldManager;
use battle::manager::BattleManager;

use crate::scene::Scene;


pub struct GameScene {

	state: SceneState,
	
	world_manager: WorldManager,
	battle_manager: BattleManager,

	party_gui: PokemonPartyGui,
	// pub pokemon_textures: PokemonTextures,
	battle_data: Option<BattleData>,

	battling: bool,

}

impl GameScene {

	pub async fn load(&mut self) {
		match postcard::from_bytes(include_bytes!("../../../build/data/world.bin")) {
			Ok(world) => {
				self.world_manager.load(world);
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
        self.world_manager.save_data(player_data.get_mut());
		info!("Saving player data!");
		if let Err(err) = save(player_data) {
			warn!("Could not save player data with error: {}", err);
		}
    }
	
}

impl Scene for GameScene {

	fn new() -> Self {
		Self {

			state: SceneState::Continue,

			world_manager: WorldManager::new(),
			battle_manager: BattleManager::new(),
			party_gui: PokemonPartyGui::new(),

			// pokemon_textures: PokemonTextures::default(),

			battle_data: None,

			battling: false,
		}
	}

	fn on_start(&mut self) {
		self.world_manager.on_start();
	}
	
	fn update(&mut self, delta: f32) {

		// Speed game up if spacebar is held down

		let delta = delta *  if is_key_down(KeyCode::Space) {
			4.0
		} else {
			1.0
		};

		// save player data if asked to

		if DIRTY.load(Relaxed) {
			if let Some(mut saves) = get_mut::<PlayerSaves>() {
				self.data_dirty(&mut saves);
			}	
		}

		if !self.battling {

			self.world_manager.update(delta, &mut self.battle_data);

			if self.battle_data.is_some() {
				if let Some(player_saves) = get::<PlayerSaves>() {
					if self.battle_manager.battle(&player_saves.get().party, self.battle_data.take().unwrap()) {
						self.battling = true;
					}
				}
			}

		} else {

			self.battle_manager.update(delta, &mut self.party_gui);
			
			if self.battle_manager.is_finished() {
				if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
					let save = player_saves.get_mut();
					if let Some(data) = self.battle_manager.update_data(save) {
						world::battle::update_world(save, data.0, data.1);
					}
				}				
				self.battling = false;
				self.world_manager.map_start(true);
			}

		}

		self.party_gui.update(delta);

	}
	
	fn render(&self) {
		if !self.battling {
			self.world_manager.render();
		} else {
			if self.battle_manager.world_active() {
				self.world_manager.render();
			}
			self.battle_manager.render();
		}
		self.party_gui.render();
	}
	
	fn input(&mut self, delta: f32) {
		if self.party_gui.is_alive() {
			self.party_gui.input();
			if pressed(Control::Start) || is_key_pressed(KeyCode::Escape) {
				self.party_gui.despawn();
				if !self.battling {
					if let Some(mut saves) = get_mut::<PlayerSaves>() {
						self.party_gui.on_finish(&mut saves.get_mut().party)
					}
				}
			}
		} else if !self.battling {
			self.world_manager.input(delta, &mut self.battle_data, &mut self.party_gui, &mut self.state);
		} else {
			self.battle_manager.input(&mut self.party_gui);
		}
	}

	fn quit(&mut self) {
		if let Some(mut player_data) = get_mut::<PlayerSaves>() {
			self.save_data(&mut player_data);
		}
		self.state = SceneState::Continue;
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}