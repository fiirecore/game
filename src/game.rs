use macroquad::prelude::collections::storage::{get, get_mut};
use crate::pokemon::pokedex::Pokedex;
use crate::util::file::PersistantData;

use crate::io::data::player::PlayerData;
use crate::battle::battle_manager::BattleManager;
use crate::util::Completable;
use crate::util::Load;
use crate::world::map::manager::WorldManager;

pub struct GameManager {

	pub(crate) world_manager: WorldManager,

	battle_manager: BattleManager,

	pokedex: Pokedex,

	//pub player_data: PlayerD,

	battling: bool,
	swapped: bool,

}

impl GameManager {

    pub fn new() -> GameManager {

		

		//let data = PlayerDataContainer::new(PlayerData::load_from_file());

		// load_player_data();

		GameManager {
			
			world_manager: WorldManager::new(&get::<PlayerData>().expect("Could not get Player Data")),

			battle_manager: BattleManager::new(),

			pokedex: Pokedex::new(),

			//player_data: data,

			battling: false,
			swapped: false,

        }
        
    }
    
    pub fn load(&mut self) {
		let mut player_data = get_mut::<PlayerData>().expect("Could not get Player Data");
		// player_data.load(); // loads gui
		if player_data.party.pokemon.len() == 0 {
			player_data.party = PlayerData::default().party;
		}		
		self.battle_manager.load();
	}

	// pub fn load_sounds(&mut self, context: &mut GameContext) {
	// 	Music::bind_world_music(context);
	// 	Music::bind_battle_music(context);
	// }

    pub fn on_start(&mut self) {
        self.world_manager.on_start();
    }

    pub fn update(&mut self, delta: f32) {

		if get::<PlayerData>().expect("Could not get Player Data").dirty {
			self.data_dirty(&mut get_mut::<PlayerData>().expect("Could not get Player Data"));
		}

		if !self.battling {

			self.world_manager.update(delta);

			if crate::util::battle_data::BATTLE_DATA.lock().is_some() {
				self.battling = true;
				self.swapped = true;
				self.battle_manager.on_start(&self.pokedex, &get::<PlayerData>().expect("Could not get Player Data"));
			}

		} else {
			if self.swapped {
				// context.battle_context.reset();
				self.swapped = false;				
			}
			self.battle_manager.update(delta, &mut get_mut::<PlayerData>().expect("Could not get Player Data"));
			if self.battle_manager.is_finished() {
				self.battling = false;
				self.swapped = true;
				self.world_manager.play_music();
			}
		}

		// if context.save_data {
		// 	context.save_data = false;
		// 	self.save_data();
		// }
		
	}
	
	pub fn render(&self) {
		if !self.battling {
			self.world_manager.render();
		} else {
			if self.battle_manager.world_active() {
				self.world_manager.render();
			}
			self.battle_manager.render();
		}
	}
	
	pub fn input(&mut self, delta: f32) {

		if !self.battling {
			self.world_manager.input(delta);
		} else {
			self.battle_manager.input(delta);
			// if context.finput.pressed(crate::util::input::Control::A) {
			// 	self.battle_intro_manager.despawn();
			// 	self.battling = false;
			// 	self.swapped = !self.swapped;
			// }
		}
	}

	pub fn quit(&mut self) {
		//self.world_manager.quit();
        self.save_data(&mut get_mut::<PlayerData>().expect("Could not get player data"));       
	}
	
	pub fn data_dirty(&mut self, player_data: &mut PlayerData) {
		self.save_data(player_data);
		player_data.dirty = false;
	}

    pub fn save_data(&mut self, player_data: &mut PlayerData) {
		//let player_data = self.player_data.get_mut();
        // player_data.world_id = self.world_manager.world_id.clone();
        self.world_manager.save_data(player_data);
		player_data.save();
		
    }
}