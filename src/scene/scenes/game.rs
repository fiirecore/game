use firecore_util::Entity;
use firecore_data::data::PersistantData;

use macroquad::prelude::collections::storage::{get, get_mut};

use crate::scene::Scene;
use crate::util::pokemon::PokemonTextures;
use super::SceneState;

use crate::world::map::manager::WorldManager;
use crate::battle::manager::BattleManager;
use crate::gui::game::party::PokemonPartyGui;

use crate::data::player::list::PlayerSaves;
use crate::battle::data::BATTLE_DATA;

pub struct GameScene {

	state: SceneState,
	
	pub world_manager: WorldManager,
	battle_manager: BattleManager,
	party_gui: PokemonPartyGui,

	pub pokemon_textures: PokemonTextures,

	battling: bool,

}

impl GameScene {
	
	pub fn new() -> Self {
		Self {

			state: SceneState::Continue,

			world_manager: WorldManager::new(),
			battle_manager: BattleManager::new(),
			party_gui: PokemonPartyGui::new(),

			pokemon_textures: PokemonTextures::default(),

			battling: false,
		}
	}

	pub fn data_dirty(&mut self, player_data: &mut PlayerSaves) {
		self.save_data(player_data);
		unsafe { crate::data::player::DIRTY = false; }
	}

    pub fn save_data(&mut self, player_data: &mut PlayerSaves) {
        self.world_manager.save_data(player_data.get_mut());
		player_data.save();
    }
	
}

#[async_trait::async_trait(?Send)]
impl Scene for GameScene {

	async fn load(&mut self) {
		
		self.world_manager.load(&mut self.battle_manager).await;
	}

	async fn on_start(&mut self) {
		self.world_manager.on_start().await;
	}
	
	fn update(&mut self, delta: f32) {

		// Speed game up if spacebar is held down

		let delta = delta *  if macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::Space) {
			4.0
		} else {
			1.0
		};

		// save player data if asked to

		if unsafe { crate::data::player::DIRTY } {
			if let Some(mut saves) = get_mut::<PlayerSaves>() {
				self.data_dirty(&mut saves);
			}	
		}

		// spawn party gui if asked to

		if unsafe { crate::gui::game::party::SPAWN } {
			unsafe { crate::gui::game::party::SPAWN = false; }
			self.party_gui.spawn();
			if self.battling {
				if let Some(party) = self.battle_manager.player_party() {
					self.party_gui.on_battle_start(&self.pokemon_textures, party);
				}				
			} else {
				self.party_gui.on_world_start(&self.pokemon_textures);
			}
		}

		if !self.battling {

			self.world_manager.update(delta);

			if unsafe { BATTLE_DATA.is_some() } {
				if let Some(player_saves) = get::<PlayerSaves>() {
					if self.battle_manager.battle(&self.pokemon_textures, &player_saves.get().party, unsafe { BATTLE_DATA.take().unwrap() }) {
						self.battling = true;
					}
				}
			}

		} else {

			self.battle_manager.update(delta, &mut self.party_gui, &self.pokemon_textures);
			
			if self.battle_manager.is_finished() {
				self.battle_manager.update_data();
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
			self.party_gui.input(delta);
			if firecore_input::pressed(firecore_input::Control::Start) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape) {
				self.party_gui.despawn();
				if !self.battling {
					if let Some(mut saves) = get_mut::<PlayerSaves>() {
						self.party_gui.on_finish(&mut saves.get_mut().party)
					}
				}
			}
		} else if !self.battling {
			self.world_manager.input(delta);
		} else {
			self.battle_manager.input(delta);
		}
	}

	fn quit(&mut self) {
		if let Some(mut player_data) = get_mut::<PlayerSaves>() {
			self.save_data(&mut player_data);
		}
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}