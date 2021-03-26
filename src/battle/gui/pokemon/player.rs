use firecore_util::{Entity, text::TextColor};
use firecore_pokedex::pokemon::battle::BattlePokemon;

use macroquad::prelude::{Vec2, Color};

use crate::util::graphics::{Texture, draw, draw_rect, draw_text_left, draw_text_right, texture::byte_texture};
use crate::gui::game::health_bar::HealthBar;

use super::PokemonGui;

pub struct PlayerPokemonGui {

	alive: bool,

	pub pos: Vec2,

	pub orig_x: f32,
	y_offset: f32,

	panel: Texture,
	name: String,
	level: String,
	health_text: String,
	health_bar: HealthBar,
	exp_width: f32,

}

impl PlayerPokemonGui {

	const EXP_COLOR: Color = macroquad::color_u8!(64, 200, 248, 255);

	pub fn new(x: f32, y: f32) -> PlayerPokemonGui {

		let ppp_x = x + super::OFFSET;

		let panel = Vec2::new(ppp_x, y);

		PlayerPokemonGui {

			alive: false,

			pos: panel,

			orig_x: x,
			y_offset: 0.0,

			panel: byte_texture(include_bytes!("../../../../build/assets/gui/battle/player_pokemon.png")),
			name: String::from("Player"),
			level: String::from("Lv"),
			health_text: String::from("/"),
			health_bar: HealthBar::new(Vec2::default(), panel + Vec2::new(super::HEALTH_X_OFFSET, super::HEALTH_Y_OFFSET)),
			exp_width: 0.0,

		}
	}

	pub fn vertical_offset(&mut self, offset: f32) {
		self.y_offset = offset;
		self.health_bar.pos.y = offset; // lol
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
		if self.is_alive() {
			self.health_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.is_alive() {
			draw(self.panel, self.pos.x, self.pos.y + self.y_offset);
			draw_text_left(0, &self.name, TextColor::Black, self.pos.x + 17.0, self.pos.y + 2.0 + self.y_offset);
			draw_text_right(0, &self.level, TextColor::Black, self.pos.x + 95.0, self.pos.y + 2.0 + self.y_offset);
			draw_text_right(0, &self.health_text, TextColor::Black, self.pos.x + 95.0, self.pos.y + 20.0 + self.y_offset);
			self.health_bar.render();
			draw_rect(Self::EXP_COLOR, self.pos.x + 32.0, self.pos.y + 33.0 + self.y_offset, self.exp_width * 64.0, 2.0);
		}		
	}

	fn update_gui(&mut self, pokemon: &BattlePokemon, new_pokemon: bool) {
		self.name = pokemon.name();
		self.level = format!("Lv{}", pokemon.data.level);
		self.exp_width = pokemon.data.experience as f32 / pokemon.pokemon.training.growth_rate.level_exp(pokemon.data.level) as f32;
		self.update_hp(new_pokemon, pokemon.current_hp, pokemon.base.hp);
	}

	fn update_hp(&mut self, new_pokemon: bool, current_health: u16, max_health: u16)  {
		self.health_bar.update_bar(new_pokemon, current_health, max_health);
		self.health_text = format!("{}/{}", current_health, max_health);
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.pos.x = x;
		self.pos.y = y;
		self.health_bar.panel.x = x + super::HEALTH_X_OFFSET;
		self.health_bar.panel.y = y + super::HEALTH_Y_OFFSET;
	}

	fn offset_position(&mut self, x: f32, y: f32) {
		self.update_position(self.pos.x + x, self.pos.y + y);
	}

}