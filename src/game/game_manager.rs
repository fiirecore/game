use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

use crate::io::data::player_data::PlayerData;
use crate::game::world::world_manager::WorldManager;
use crate::game::battle::battle_manager::BattleManager;
use crate::game::pokedex::pokedex::Pokedex;

use crate::entity::entity::{Entity, Ticking};
use crate::io::data::player_data::SavedPokemon;
use crate::util::traits::Loadable;
use crate::util::traits::PersistantData;
use super::battle::transitions::managers::battle_intro_manager::BattleIntroManager;
use super::battle::transitions::traits::battle_transition_manager::BattleTransitionManager;
use super::pokedex::pokemon::pokemon_owned::OwnedPokemon;

pub struct GameManager {

    world_manager: WorldManager,
	battle_manager: BattleManager,
	pokedex: Pokedex,

	pub player_data: PlayerData,

	battling: bool,
	battle_type: u8,
	swapped: bool,

	battle_intro_manager: BattleIntroManager,

	pkmn_instance: Option<SavedPokemon>,

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
			battle_type: 0,
			swapped: false,

			battle_intro_manager: BattleIntroManager::new(),

			pkmn_instance: None,

        }
        
    }
    
    pub fn load(&mut self) {
		self.pokedex.load();
		if self.player_data.party.pokemon.len() == 0 {
			self.player_data.add_pokemon_to_party(OwnedPokemon::get_default(&self.pokedex));
		}		
		self.battle_manager.load();
		self.world_manager.load(&self.player_data);
		self.battle_intro_manager.load();
		self.battle_intro_manager.load_intros();
    }

    pub fn on_start(&mut self, context: &mut GameContext) {
        self.world_manager.on_start(context);
    }

    pub fn update(&mut self, context: &mut GameContext) {

		if self.battle_intro_manager.is_finished() {
			self.battle_intro_manager.despawn();
			match self.battle_type {
				0 => {
					self.battle_manager.generate_test_battle(&self.pokedex, context, &self.player_data);
					
				},
				1 => {
					if self.world_manager.world_map_manager.is_alive() {
						self.battle_manager.generate_wild_battle(self.player_data.party.pokemon[0].to_instance(&self.pokedex), &mut self.world_manager.world_map_manager.get_current_world_mut().get_current_piece_mut().wild_pokemon_table.as_mut().unwrap(), &self.pokedex, context);
						self.world_manager.world_map_manager.generate_a_battle = false;
					} else {
						self.battle_manager.generate_wild_battle(self.player_data.party.pokemon[0].to_instance(&self.pokedex), &mut self.world_manager.warp_map_manager.current_map_set_mut().current_map_mut().wild_pokemon_table.as_mut().unwrap(), &self.pokedex, context);
						self.world_manager.warp_map_manager.generate_a_battle = false;
					}
					
					//self.battle_manager.new_battle(pp, self.world_manager.world_map_manager.generate_wild_battle(context, &self.pokedex));
					self.battle_type = 0;
				},
				2 => {
					self.battle_manager.new_battle(self.player_data.party.pokemon[0].to_instance(&self.pokedex), self.pkmn_instance.as_ref().unwrap().to_instance(&self.pokedex).instance);
					self.pkmn_instance = None;
					self.battle_type = 0;
				}
				_ => {

				}
			}			
			self.battling = true;
			self.swapped = true;
		}

		if !self.battling {
			if self.swapped {
				self.world_manager.on_start(context);
				self.swapped = false;
			}
			if self.battle_intro_manager.is_alive() {
				self.battle_intro_manager.update(context);
			}
			self.world_manager.update(context);
		} else {
			if self.swapped {
				self.battle_manager.on_start(context);
				self.swapped = false;
			}
			self.battle_manager.update(context);
			if self.battle_manager.finished {
				self.battling = false;
				self.swapped = true;
			}
		}

		if !(self.battling || self.battle_intro_manager.is_alive()) {
			if self.world_manager.world_map_manager.generate_a_battle || self.world_manager.warp_map_manager.generate_a_battle { // remove for better code
				self.battle_type = 1;
				self.battle_intro_manager.spawn();
				self.battle_intro_manager.on_start(context);
			}
			if context.battle.is_some() {
				self.battle_type = 2;
				self.pkmn_instance = context.battle.clone();
				context.battle = None;
				self.battle_intro_manager.spawn();
				self.battle_intro_manager.on_start(context);
			}
		}
		
		
	}
	
	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if !self.battling {
			self.world_manager.render(ctx, g, tr);
			if self.battle_intro_manager.is_alive() {
				self.battle_intro_manager.render(ctx, g, tr);
			}
		} else {
			self.battle_manager.render(ctx, g, tr);
		}
	}
	
	pub fn input(&mut self, context: &mut GameContext) {
		if !(self.battling || self.battle_intro_manager.is_alive()) {
			if context.fkeys[0] == 1 {
				self.battle_intro_manager.spawn();		
				self.battle_intro_manager.on_start(context);
			}
			self.world_manager.input(context);
		} else {
			self.battle_manager.input(context);
			if context.fkeys[0] == 1 {
				self.battle_intro_manager.despawn();
				self.battling = false;
				self.swapped = !self.swapped;
			}
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