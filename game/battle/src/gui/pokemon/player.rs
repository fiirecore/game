use game::{
	util::{Entity, Reset},
	pokedex::pokemon::instance::PokemonInstance,
	macroquad::prelude::{Vec2, Texture2D},
	gui::health::HealthBar,
	text::TextColor,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use super::{PokemonGui, PokemonGuiOffset};
use crate::gui::exp_bar::ExperienceBar;

pub struct PlayerPokemonGui {

	alive: bool,

	origin: Vec2,
	offset: PokemonGuiOffset,
	bounce: f32,

	background: Texture2D,

	name: String,
	level: String,
	health_text: String,
	health_bar: HealthBar,
	exp_bar: ExperienceBar,

}

impl PlayerPokemonGui {

	const OFFSET: Vec2 = game::macroquad::prelude::const_vec2!([super::OFFSET, 0.0]);

	pub fn new(origin: Vec2) -> PlayerPokemonGui {

		PlayerPokemonGui {

			alive: false,

			origin,
			offset: PokemonGuiOffset::new(),
			bounce: 0.0,

			background: byte_texture(include_bytes!("../../../assets/gui/player_pokemon.png")),
			name: String::new(),
			level: String::new(),
			health_text: String::new(),
			health_bar: HealthBar::new(origin + Vec2::new(48.0, super::HEALTH_Y_OFFSET), Self::OFFSET),
			exp_bar: ExperienceBar::new(origin + Vec2::new(32.0, 33.0), Self::OFFSET),

		}
	}

	pub fn vertical_offset(&mut self, offset: f32) {
		self.bounce = offset;
		self.health_bar.offset.y = offset;
	}

}

impl Entity for PlayerPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.health_bar.spawn();
		self.offset.reset();
		self.bounce = 0.0;
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.health_bar.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for PlayerPokemonGui {

	fn update(&mut self, delta: f32) {
		if self.alive {
			self.health_bar.update(delta);
			self.exp_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.alive {
			let x = self.origin.x + self.offset.x;
			let y = self.origin.y + self.bounce;
			draw(self.background, x, y);
			let x2 = x + 95.0;
			let y2 = y + 2.0;
			draw_text_left(0, &self.name, TextColor::Black, x + 17.0, y2);
			draw_text_right(0, &self.level, TextColor::Black, x2, y2);
			draw_text_right(0, &self.health_text, TextColor::Black, x2, y + 20.0);
			self.health_bar.render();
			self.exp_bar.render(y);
		}		
	}

	fn update_gui(&mut self, pokemon: &PokemonInstance, new_pokemon: bool) {

		self.name = pokemon.name();
		self.level = format!("Lv{}", pokemon.data.level);
		self.health_bar.update_bar(new_pokemon, pokemon.current_hp, pokemon.base.hp);
		self.health_text = format!("{}/{}", pokemon.current_hp, pokemon.base.hp);

		self.exp_bar.update_exp(pokemon, new_pokemon);

	}

	fn offset(&mut self, delta: f32) -> bool {
		if self.alive {
			let done = self.offset.update(delta);
			self.health_bar.offset.x = self.offset.x;
			done
		} else {
			false
		}
	}

}