use crate::gui::GuiComponent;
use macroquad::prelude::collections::storage::{get, get_mut};
use crate::battle::battle_manager::BattleManager;
use crate::gui::game::pokemon_party_gui::PokemonPartyGui;
use crate::scene::Scene;
use firecore_util::Completable;
use firecore_data::data::PersistantData;
use crate::world::map::manager::WorldManager;
use firecore_util::Entity;

use crate::data::player::list::PlayerSaves;

use super::SceneState;

pub struct GameScene {

	state: SceneState,
	
	pub world_manager: WorldManager,
	battle_manager: BattleManager,
	party_gui: PokemonPartyGui,

	battling: bool,
	swapped: bool,

}

impl GameScene {
	
	pub fn new() -> GameScene {
		GameScene {

			state: SceneState::Continue,

			world_manager: WorldManager::new(),
			battle_manager: BattleManager::new(),
			party_gui: PokemonPartyGui::new(),

			battling: false,
			swapped: false,
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
		self.world_manager.load().await;
	}

	async fn on_start(&mut self) {
		self.world_manager.on_start().await;
	}
	
	fn update(&mut self, delta: f32) {
		let delta = delta *  if macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::Space) {
			4.0
		} else {
			1.0
		};
		if unsafe { crate::data::player::DIRTY } {
			if let Some(mut saves) = get_mut::<PlayerSaves>() {
				self.data_dirty(&mut saves);
			}	
		}
		if let Some(despawn_on_select) = unsafe { crate::gui::game::pokemon_party_gui::SPAWN.take() } {
			self.party_gui.spawn();
			if self.battling {
				self.party_gui.on_battle_start(&self.battle_manager.current_battle.player_pokemon);
			} else {
				self.party_gui.on_world_start(despawn_on_select);
			}
		}

		if !self.battling {

			self.world_manager.update(delta);

			if crate::util::battle_data::BATTLE_DATA.lock().is_some() {
				if let Some(player_saves) = get::<PlayerSaves>() {
					self.battling = true;
					self.swapped = true;
					self.battle_manager.on_start(&player_saves.get().party, crate::util::battle_data::BATTLE_DATA.lock().take().unwrap());
				}
			}

		} else {
			if self.swapped {
				// context.battle_context.reset();
				self.swapped = false;				
			}
			self.battle_manager.update(delta, &mut self.party_gui);
			if self.battle_manager.is_finished() {
				if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
					self.battle_manager.current_battle.update_data(player_saves.get_mut());
				}
				self.battling = false;
				self.swapped = true;
				self.world_manager.play_music();
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