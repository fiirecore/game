use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

use crate::io::data::player_data::PlayerData;
use crate::game::world::world_manager::WorldManager;
use crate::game::battle::battle_manager::BattleManager;
use crate::game::pokedex::pokedex::Pokedex;

use crate::entity::entity::Entity;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use crate::util::traits::PersistantData;

pub struct GameManager {

	world_manager: WorldManager,

	battle_manager: BattleManager,

	pokedex: Pokedex,

	pub player_data: PlayerData,

	battling: bool,
	swapped: bool,

}

impl GameManager {

    pub fn new() -> GameManager {

		let player_data =  PlayerData::load_from_file();

		GameManager {
			
			world_manager: WorldManager::new(&player_data),

			battle_manager: BattleManager::new(),

			pokedex: Pokedex::new(),

			player_data: player_data,

			battling: false,
			swapped: false,

        }
        
    }
    
    pub fn load(&mut self) {
		self.pokedex.load();
		if self.player_data.party.pokemon.len() == 0 {
			self.player_data.default_add(&self.pokedex);
		}		
		self.battle_manager.load();
		self.world_manager.load(&self.player_data);
    }

    pub fn on_start(&mut self, context: &mut GameContext) {
        self.world_manager.on_start(context);
    }

    pub fn update(&mut self, context: &mut GameContext) {

		if !self.battling {

			self.world_manager.update(context);

			if context.battle_context.battle {
				self.battling = true;
				self.swapped = true;
				self.battle_manager.on_start(context, &self.pokedex, &self.player_data);
			}

		} else {
			if self.swapped {
				context.battle_context.reset();
				self.swapped = false;				
			}
			self.battle_manager.update(context, &mut self.player_data);
			if self.battle_manager.is_finished() {
				self.battling = false;
				self.world_manager.on_start(context);
				self.swapped = true;
			}
		}		
		
	}
	
	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if !self.battling {
			self.world_manager.render(ctx, g, tr);
		} else {
			if self.battle_manager.world_active() {
				self.world_manager.render(ctx, g, tr);
			}
			self.battle_manager.render(ctx, g, tr);
		}
	}
	
	pub fn input(&mut self, context: &mut GameContext) {
		if !self.battling {
			self.world_manager.input(context);
		} else {
			self.battle_manager.input(context);
			// if context.fkeys[0] == 1 {
			// 	self.battle_intro_manager.despawn();
			// 	self.battling = false;
			// 	self.swapped = !self.swapped;
			// }
		}
	}

	pub fn dispose(&mut self) {
		self.world_manager.dispose();
        self.save_data();       
    }

    pub fn save_data(&mut self) {
        self.player_data.location.world_id = self.world_manager.world_id.clone();
        if self.world_manager.world_map_manager.is_alive() {
            self.player_data.location.map_set_id = String::from("world");
            self.player_data.location.map_set_num = 0;
        } else {
            self.player_data.location.map_set_id = self.world_manager.warp_map_manager.current_map_set_id.clone();
            self.player_data.location.map_set_num = self.world_manager.warp_map_manager.map_sets.get(&self.world_manager.warp_map_manager.current_map_set_id).unwrap().current_map_index;
        }
        self.world_manager.player.save_data(&mut self.player_data);
        self.player_data.save();
    }
}