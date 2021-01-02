use std::fmt::Display;

use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

use crate::game::pokedex::pokemon_move::move_instance::MoveInstance;
use crate::game::pokedex::pokemon_move::move_category::MoveCategory;
use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
use crate::game::pokedex::pokemon::pokemon_owned::OwnedPokemon;
use crate::game::battle::battle_pokemon::*;

pub struct Battle {
	
	pub player_pokemon: BattlePokemon,
	pub opponent_pokemon: BattlePokemon,

	pub finished: bool,
	
}

impl Battle {
	
	pub fn empty() -> Self {
		
		Self {
		
			player_pokemon: BattlePokemon::empty(),
			opponent_pokemon:BattlePokemon::empty(),

			finished: false,
		
		}
		
	}
	
	pub fn new(player_pokemon: OwnedPokemon, opponent_pokemon: PokemonInstance) -> Self {
		
		Self {
			
			player_pokemon: BattlePokemon::new(player_pokemon.instance),
			opponent_pokemon: BattlePokemon::new(opponent_pokemon),

			finished: false,
			
		}
		
	}

	fn get_move_damage(&self, pmove: &MoveInstance, pokemon: &BattlePokemon, recieving_pokemon: &BattlePokemon) -> usize {
		let level = pokemon.instance.as_ref().unwrap().level;
		if let Some(power) = pmove.move_instance.power {
			match pmove.move_instance.category {
				MoveCategory::Status => return 0,
				MoveCategory::Physical => {
					return (((2.0 * level as f64 / 5.0 + 2.0).floor() * pokemon.atk as f64 * power as f64 / recieving_pokemon.def as f64).floor() / 50.0).floor() as usize + 2;
				},
				MoveCategory::Special => {
					return (((2.0 * level as f64 / 5.0 + 2.0).floor() * pokemon.sp_atk as f64 * power as f64 / recieving_pokemon.sp_def as f64).floor() / 50.0).floor() as usize + 2;
				}
			}
		} else {
			return 0;
		}		
	}


	pub fn load(&mut self) {

		self.player_pokemon.load_texture(false);
		self.opponent_pokemon.load_texture(true);
		
	}
	
	pub fn update(&mut self, _context: &mut GameContext) {
		
	}
	
	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer, offset: u16, ppp_y_o: u8) {
		self.opponent_pokemon.render(ctx, g, 144 - offset as isize, 74);
		self.player_pokemon.render(ctx, g, 40 + offset as isize, 113 + ppp_y_o as isize);
		//draw_o(ctx, g, &self.opponent_pokemon_texture, 144 - offset as isize, 74 - self.opponent_y_offset as isize); // fix with offset
		//draw_o(ctx, g, &self.player_pokemon_texture, 40 + offset as isize, 113 - self.player_y_offset as isize + ppp_y_o as isize);
	}

	pub fn player_move(&mut self, index: usize) -> String {
		if let Some(pmove) = self.player_pokemon.instance.as_ref().unwrap().get_move(index) {
			let damage = self.get_move_damage(pmove, &self.player_pokemon, &self.opponent_pokemon);
			if damage >= self.opponent_pokemon.current_hp {
				self.opponent_pokemon.faint = true;
			} else {
				self.opponent_pokemon.current_hp -= damage;
			}
			return pmove.move_instance.name.clone();
		}
		return String::from("None");
	}

	pub fn opponent_move(&mut self, index: usize) -> String {
		if let Some(pmove) = self.opponent_pokemon.instance.as_ref().unwrap().get_move(index) {
			let damage = self.get_move_damage(pmove, &self.opponent_pokemon, &self.player_pokemon);
			if damage >= self.player_pokemon.current_hp {
				self.player_pokemon.faint = true;
			} else {
				self.player_pokemon.current_hp -= damage;
			}
			return pmove.move_instance.name.clone();
		}	
		return String::from("None");
	}

	pub fn run(&mut self) {
		self.finished = true;
	}
	
//	fn render_above(&mut self, c: &mut Context, g: &mut GlGraphics) {}
	
}

impl Display for Battle {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs. {}", self.player_pokemon, self.opponent_pokemon)
    }
}