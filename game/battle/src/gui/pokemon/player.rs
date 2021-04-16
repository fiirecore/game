use game::{
	util::{Entity, text::TextColor},
	pokedex::pokemon::instance::PokemonInstance,
	macroquad::prelude::{Vec2, Texture2D},
	gui::health_bar::HealthBar,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use crate::gui::exp_bar::ExperienceBar;
use super::PokemonGui;

pub struct PlayerPokemonGui {

	alive: bool,

	pub pos: Vec2,

	pub orig_x: f32,
	y_offset: f32,

	panel: Texture2D,
	name: String,
	level: String,
	health_text: String,
	health_bar: HealthBar,
	exp_bar: ExperienceBar,

}

impl PlayerPokemonGui {

	pub fn new(x: f32, y: f32) -> PlayerPokemonGui {

		let ppp_x = x + super::OFFSET;

		let panel = Vec2::new(ppp_x, y);

		PlayerPokemonGui {

			alive: false,

			pos: panel,

			orig_x: x,
			y_offset: 0.0,

			panel: byte_texture(include_bytes!("../../../assets/gui/player_pokemon.png")),
			name: String::from("Player"),
			level: String::from("Lv"),
			health_text: String::from("/"),
			health_bar: HealthBar::new(Vec2::new(super::HEALTH_X_OFFSET, super::HEALTH_Y_OFFSET), panel),
			exp_bar: ExperienceBar::new(Vec2::new(32.0, 33.0), panel),

		}
	}

	pub fn vertical_offset(&mut self, offset: f32) {
		self.y_offset = offset;
	}

}

impl Entity for PlayerPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.health_bar.spawn();
		self.reset();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.health_bar.despawn();
		self.reset();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for PlayerPokemonGui {

	fn reset(&mut self) {
		self.update_position(self.orig_x + super::OFFSET, self.pos.y);
	}

	fn update(&mut self, delta: f32) {
		if self.alive {
			self.health_bar.update(delta);
			self.exp_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.alive {
			draw(self.panel, self.pos.x, self.pos.y + self.y_offset);
			draw_text_left(0, &self.name, TextColor::Black, self.pos.x + 17.0, self.pos.y + 2.0 + self.y_offset);
			draw_text_right(0, &self.level, TextColor::Black, self.pos.x + 95.0, self.pos.y + 2.0 + self.y_offset);
			draw_text_right(0, &self.health_text, TextColor::Black, self.pos.x + 95.0, self.pos.y + 20.0 + self.y_offset);
			self.health_bar.render(self.y_offset);
			self.exp_bar.render(self.y_offset);
		}		
	}

	fn update_gui(&mut self, pokemon: &PokemonInstance, new_pokemon: bool) {

		self.name = pokemon.name();
		self.level = format!("Lv{}", pokemon.data.level);
		self.health_bar.update_bar(new_pokemon, pokemon.current_hp, pokemon.base.hp);
		self.health_text = format!("{}/{}", pokemon.current_hp, pokemon.base.hp);

		self.exp_bar.update_exp(pokemon, new_pokemon);

	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.pos.x = x;
		self.pos.y = y;
		self.health_bar.panel.x = x;
		self.health_bar.panel.y = y;
		self.exp_bar.panel.x = x;
		self.exp_bar.panel.y = y;
	}

	fn offset_position(&mut self, x: f32, y: f32) {
		self.update_position(self.pos.x + x, self.pos.y + y);
	}

}