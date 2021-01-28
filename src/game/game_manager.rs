use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;
use crate::util::input::Control;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::io::data::player_data::PlayerData;
use crate::battle::battle_manager::BattleManager;
use crate::game::pokedex::pokedex::Pokedex;
use crate::util::Completable;
use crate::util::Load;
use crate::world::map::manager::WorldManager;
use super::player_data_container::PlayerDataContainer;

pub struct GameManager {

	pub(crate) world_manager: WorldManager,

	battle_manager: BattleManager,

	pokedex: Pokedex,

	pub player_data: PlayerDataContainer,

	battling: bool,
	swapped: bool,

}

impl GameManager {

    pub async fn new() -> GameManager {

		let data = PlayerDataContainer::new(PlayerData::load_from_file());

		GameManager {
			
			world_manager: WorldManager::new(data.get()).await,

			battle_manager: BattleManager::new(),

			pokedex: Pokedex::new(),

			player_data: data,

			battling: false,
			swapped: false,

        }
        
    }
    
    pub fn load(&mut self) {
		self.player_data.load(); // loads gui
		if self.player_data.get().party.pokemon.len() == 0 {
			self.player_data.get_mut().default_add(&self.pokedex);
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

		if !self.battling {

			self.world_manager.update(delta);

			if crate::util::battle_data::BATTLE_DATA.lock().is_some() {
				self.battling = true;
				self.swapped = true;
				self.battle_manager.on_start(&self.pokedex, self.player_data.get());
			}

		} else {
			if self.swapped {
				// context.battle_context.reset();
				self.swapped = false;				
			}
			self.battle_manager.update(delta, self.player_data.get_mut());
			if self.battle_manager.is_finished() {
				self.battling = false;
				self.swapped = true;
				//context.stop_music();
				self.world_manager.play_music();
			}
		}

		// if context.save_data {
		// 	context.save_data = false;
		// 	self.save_data();
		// }
		
	}
	
	pub fn render(&self, tr: &TextRenderer) {
		if !self.battling {
			self.world_manager.render(tr);
		} else {
			if self.battle_manager.world_active() {
				self.world_manager.render(tr);
			}
			self.battle_manager.render(tr);
		}
	}
	
	pub fn input(&mut self, delta: f32) {

		if crate::util::input::pressed(Control::Start) {
			self.save_data();
		}

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
		//self.world_manager.dispose();
        self.save_data();       
    }

    pub fn save_data(&mut self) {
		macroquad::prelude::info!("Saving player data...");
		let player_data = self.player_data.get_mut();
        // player_data.world_id = self.world_manager.world_id.clone();
        if self.world_manager.chunk_map.is_alive() {
            player_data.location.map_id = String::from("world");
            player_data.location.map_index = 0;
        } else {
            player_data.location.map_id = self.world_manager.map_sets.get().clone();
            player_data.location.map_index = *self.world_manager.map_sets.get_index() as u16;
		}
		player_data.location.position = self.world_manager.player.position;
        player_data.save();
    }
}