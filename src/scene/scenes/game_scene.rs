use macroquad::prelude::collections::storage::get;
use macroquad::prelude::collections::storage::get_mut;

use crate::battle::battle_manager::BattleManager;
use crate::io::data::player::PlayerData;
use crate::scene::Scene;
use crate::util::Completable;
use crate::util::file::PersistantData;
use crate::world::map::manager::WorldManager;

use super::Scenes;

pub struct GameScene {

	scene_token: Option<Scenes>,
	
	world_manager: WorldManager,
	battle_manager: BattleManager,

	battling: bool,
	swapped: bool,

}

impl GameScene {
	
	pub fn new() -> GameScene {
		GameScene {
			scene_token: None,

			world_manager: WorldManager::new(),
			battle_manager: BattleManager::new(),

			battling: false,
			swapped: false,
		}
	}

	pub fn data_dirty(&mut self, player_data: &mut PlayerData) {
		self.save_data(player_data);
		player_data.dirty = false;
	}

    pub fn save_data(&mut self, player_data: &mut PlayerData) {
        self.world_manager.save_data(player_data);
		player_data.save();
    }
	
}

impl Scene for GameScene {

	fn on_start(&mut self) {
		self.world_manager.on_start();
	}
	
	fn update(&mut self, delta: f32) {
		if get::<PlayerData>().expect("Could not get Player Data").dirty {
			self.data_dirty(&mut get_mut::<PlayerData>().expect("Could not get Player Data"));
		}

		if !self.battling {

			self.world_manager.update(delta);

			if crate::util::battle_data::BATTLE_DATA.lock().is_some() {
				self.battling = true;
				self.swapped = true;
				self.battle_manager.on_start(&get::<PlayerData>().expect("Could not get Player Data"), crate::util::battle_data::BATTLE_DATA.lock().take().unwrap());
			}

		} else {
			if self.swapped {
				// context.battle_context.reset();
				self.swapped = false;				
			}
			self.battle_manager.update(delta);
			if self.battle_manager.is_finished() {
				self.battle_manager.current_battle.update_data(&mut get_mut::<PlayerData>().expect("Could not get Player Data"));
				self.battling = false;
				self.swapped = true;
				self.world_manager.play_music();
			}
		}
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
	}
	
	fn input(&mut self, delta: f32) {
		if !self.battling {
			self.world_manager.input(delta);
		} else {
			self.battle_manager.input(delta);
			// if context.finput.pressed(crate::io::input::Control::A) {
			// 	self.battle_intro_manager.despawn();
			// 	self.battling = false;
			// 	self.swapped = !self.swapped;
			// }
		}
	}

	fn quit(&mut self) {
        self.save_data(&mut get_mut::<PlayerData>().expect("Could not get player data"));      
	}
	
	fn next_scene(&self) -> Option<Scenes> {
		self.scene_token
	}
	
}