use std::fmt::Display;

use opengl_graphics::GlGraphics;
use piston_window::Context;

use opengl_graphics::ImageSize;
use opengl_graphics::Texture;

use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw_o;
use crate::util::texture_util::texture64_from_path;

pub struct BattlePokemon {
	
	pub instance: Option<PokemonInstance>,

	pub texture: Option<Texture>,
	pub y_offset: u8,
	
	pub current_hp: usize,
	
	pub hp: usize,
	pub atk: usize,
	pub def: usize,
	pub sp_atk: usize,
	pub sp_def: usize,
	pub speed: usize,

	pub faint: bool,
	
}

impl BattlePokemon {
	

	pub fn empty() -> BattlePokemon {
		
		BattlePokemon {
			
			instance: None,

			texture: None,
			y_offset: 0,

			current_hp: 0,
			
			hp: 0,
			atk: 0,
			def: 0,
			sp_atk: 0,
			sp_def: 0,
			speed: 0,

			faint: false,
			
		}		
		
	}
	
	pub fn new(pokemon: PokemonInstance) -> BattlePokemon {
		
		let hp = calculate_hp(pokemon.pokemon.base_hp, pokemon.ivs.hp, pokemon.evs.hp, pokemon.level);
		let atk = calculate_stat(pokemon.pokemon.base_atk, pokemon.ivs.atk, pokemon.evs.atk, pokemon.level);
		let def = calculate_stat(pokemon.pokemon.base_def, pokemon.ivs.def, pokemon.evs.def, pokemon.level);
		let sp_atk = calculate_stat(pokemon.pokemon.base_sp_atk, pokemon.ivs.sp_atk, pokemon.evs.sp_atk, pokemon.level);
		let sp_def = calculate_stat(pokemon.pokemon.base_sp_def, pokemon.ivs.sp_def, pokemon.evs.sp_def, pokemon.level);
		let speed = calculate_stat(pokemon.pokemon.base_speed, pokemon.ivs.speed, pokemon.evs.speed, pokemon.level);
		
		BattlePokemon {

			instance: Some(pokemon),
			texture: None,
			y_offset: 0,
			
			current_hp: hp,
						
			hp: hp,
			atk: atk,
			def: def,
			sp_atk: sp_atk,
			sp_def: sp_def,
			speed: speed,

			faint: false,
			
		}
		
	}

	pub fn load_texture(&mut self, front: bool) {
		if front {
			self.texture = Some(texture64_from_path(asset_as_pathbuf(&self.instance.as_ref().unwrap().pokemon.path_normal_front.as_str())));
		} else {
			self.texture = Some(texture64_from_path(asset_as_pathbuf(&self.instance.as_ref().unwrap().pokemon.path_normal_back.as_str())));
		}
		self.y_offset = self.texture.as_ref().unwrap().get_height() as u8;
	}

	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, x: isize, y: isize) {
		draw_o(ctx, g, &self.texture, x, y - self.y_offset as isize);
	}
	
}

pub fn calculate_stat(base_stat: u8, iv_stat: u8, ev_stat: u8, level: u8) -> usize { //add item check
 	let nature = 1.0;
	(((2.0 * base_stat as f64 + iv_stat as f64 + ev_stat as f64) * level as f64 / 100.0 + 5.0).floor() * nature).floor() as usize
}

pub fn calculate_hp(base_hp: u8, iv_hp: u8, ev_hp: u8, level: u8) -> usize {
	((2.0 * base_hp as f64 + iv_hp as f64 + ev_hp as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as usize
}

impl Display for BattlePokemon {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.instance.as_ref().unwrap().level, &self.instance.as_ref().unwrap().pokemon.name)
	}
	
}