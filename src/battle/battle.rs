use std::fmt::Display;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;
use crate::engine::game_context::GameContext;

use crate::entity::entity::Entity;
use crate::game::pokedex::pokedex::Pokedex;
use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
use crate::game::pokedex::pokemon::pokemon_owned::OwnedPokemon;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::battle::battle_text;
use crate::gui::gui::GuiComponent;
use crate::io::data::pokemon::moves::MoveCategory;
use crate::io::data::pokemon::moves::pokemon_move::PokemonMove;
use crate::io::data::pokemon::pokemon::Pokemon;
use crate::io::data::pokemon::pokemon_party::PokemonParty;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw_bottom;
use crate::util::texture_util::texture64_from_path;

use super::transitions::managers::battle_closer_manager::BattleCloserManager;

pub struct Battle {
	
	pub player_pokemon: Vec<OwnedPokemon>,
	pub opponent_pokemon: Vec<PokemonInstance>,

	pub player_active: usize,
	pub opponent_active: usize,

	pub player_move: PokemonMove,
	pub opponent_move: PokemonMove,

	pub player_textures: Vec<Texture>,
	pub opponent_textures: Vec<Texture>,

	//pub battle_events: BattleEventManager,

	pub pmove_queued: bool,
	pub omove_queued: bool,
	pub faint_queued: bool,

	//pub move_finished: bool,
	pub faint: bool,
	
}

impl Default for Battle {
	fn default() -> Self {
		
		Self {
		
			player_pokemon: Vec::new(),
			opponent_pokemon: Vec::new(),

			player_active: 0,
			opponent_active: 0,

			player_move: PokemonMove::empty(),
			opponent_move: PokemonMove::empty(),

			player_textures: Vec::new(),
			opponent_textures: Vec::new(),

			pmove_queued: false,
			omove_queued: false,
			faint_queued: false,

			//move_finished: false,
			faint: false,
		
		}
		
	}
}

impl Battle {
	
	pub fn new(pokedex: &Pokedex, player_pokemon: &PokemonParty, opponent_pokemon: &PokemonParty) -> Self {
		
		Self {
			
			player_pokemon: player_pokemon.pokemon.iter().map(|pkmn|
				pkmn.to_owned_pokemon(pokedex)
			).collect(),
			opponent_pokemon: opponent_pokemon.to_instance(pokedex),
			
			..Battle::default()
			
		}
		
	}

	fn load_textures(&mut self) {
		for i in &self.opponent_pokemon {
			self.opponent_textures.push(texture64_from_path(asset_as_pathbuf(Pokemon::texture_path("front", &i.pokemon))));
		}
		for i in &self.player_pokemon {
			self.player_textures.push(texture64_from_path(asset_as_pathbuf(Pokemon::texture_path("back", &i.instance.pokemon))));
		}
	}

	pub fn load(&mut self) {
		self.load_textures();
	}

	pub fn update(&mut self, context: &mut GameContext, battle_gui: &mut BattleGui, battle_closer_manager: &mut BattleCloserManager) {
		if self.pmove_queued || self.omove_queued || self.faint_queued {
			if battle_gui.opponent_pokemon_gui.health_bar.get_width() == 0 {
				battle_gui.update_gui(&self);
			}
			if self.player().base.speed > self.opponent().base.speed {
				battle_text::pmove(self, battle_gui);
			} else {
				battle_text::omove(self, battle_gui);
			}
		} else if self.faint {
			if self.player().current_hp == 0 {
				for pkmn_index in 0..self.player_pokemon.len() {
					if self.player_pokemon[pkmn_index].instance.current_hp != 0 {
						self.faint = false;
						self.player_active = pkmn_index;
						battle_gui.update_gui(&self);
						break;
					}
				}
				if self.faint {
					battle_closer_manager.spawn();
				}
			} else {
				for pkmn_index in 0..self.opponent_pokemon.len() {
					if self.opponent_pokemon[pkmn_index].current_hp != 0 {
						self.faint = false;
						self.opponent_active = pkmn_index;
						battle_gui.update_gui(&self);
						break;
					}
				}
				if self.faint {
					battle_closer_manager.spawn();
				}
			}
		} else if !(battle_gui.player_panel.battle_panel.is_active() || battle_gui.player_panel.fight_panel.is_active()) {
			//self.finished = false;
			battle_gui.player_panel.start();
		}
	}
	
	pub fn render(&self, ctx: &mut Context, g: &mut GlGraphics, offset: u16, ppp_y_o: u8) {
		draw_bottom(ctx, g, &self.opponent_textures[self.opponent_active], 144 - offset as isize, 74);
		draw_bottom(ctx, g, &self.player_textures[self.player_active], 40 + offset as isize, 113 + ppp_y_o as isize);
	}

	pub fn queue_player_move(&mut self, index: usize) {
		self.player_move = self.player_mut().moves[index].use_move();
	}

	pub fn queue_opponent_move(&mut self, context: &mut GameContext) {
		let index = context.random.rand_range(0..self.opponent().moves.len() as u32) as usize;
		self.opponent_move = self.opponent_mut().moves[index].use_move();
	}

	pub fn queue_faint(&mut self) {
		self.omove_queued = false;
		self.pmove_queued = false;
		self.faint_queued = true;
	}

	pub fn player_move(&mut self) {
		let damage = get_move_damage(&self.player_move, &self.player_pokemon[self.player_active].instance, self.opponent());
		let opponent = &mut self.opponent_pokemon[self.opponent_active];
		if damage >= opponent.current_hp {
			opponent.current_hp = 0;
		} else {
			opponent.current_hp -= damage;
		}
	}

	pub fn opponent_move(&mut self) {
		let damage = get_move_damage(&self.opponent_move, &self.opponent_pokemon[self.opponent_active], &self.player_pokemon[self.player_active].instance);
		let player = &mut self.player_pokemon[self.player_active].instance;
		if damage >= player.current_hp {
			player.current_hp = 0;
		} else {
			player.current_hp -= damage;
		}
	}

	pub fn player(&self) -> &PokemonInstance {
		&self.player_pokemon[self.player_active].instance
	}

	pub fn player_mut(&mut self) -> &mut PokemonInstance {
		&mut self.player_pokemon[self.player_active].instance
	}

	pub fn opponent(&self) -> &PokemonInstance {
		&self.opponent_pokemon[self.opponent_active]
	}

	pub fn opponent_mut(&mut self) -> &mut PokemonInstance {
		&mut self.opponent_pokemon[self.opponent_active]
	}

	pub fn run(&mut self) {
		//self.finished = true;
	}
	
//	fn render_above(&mut self, c: &mut Context, g: &mut GlGraphics) {}
	
}

pub struct BattleEndData {

}




fn get_move_damage(pmove: &PokemonMove, pokemon: &PokemonInstance, recieving_pokemon: &PokemonInstance) -> u16 {
	if let Some(power) = pmove.power {
		match pmove.category {
			MoveCategory::Status => return 0,
			MoveCategory::Physical => {
				return (((2.0 * pokemon.level as f64 / 5.0 + 2.0).floor() * pokemon.base.atk as f64 * power as f64 / recieving_pokemon.base.def as f64).floor() / 50.0).floor() as u16 + 2;
			},
			MoveCategory::Special => {
				return (((2.0 * pokemon.level as f64 / 5.0 + 2.0).floor() * pokemon.base.sp_atk as f64 * power as f64 / recieving_pokemon.base.sp_def as f64).floor() / 50.0).floor() as u16+ 2;
			}
		}
	} else {
		return 0;
	}		
}

impl Display for Battle {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs. {}", self.player(), self.opponent())
    }
}