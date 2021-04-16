use game::{
	util::{Entity, text::TextColor},
	pokedex::pokemon::instance::PokemonInstance,
	macroquad::prelude::{Vec2, Texture2D},
	gui::health_bar::HealthBar,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use super::PokemonGui;

pub struct OpponentPokemonGui {

	alive: bool,

	pub pos: Vec2,

	pub orig_x: f32,

	panel: Texture2D,
	name: String,
	level: String,
	health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(x: f32, y: f32) -> OpponentPokemonGui {

		let x_offset = x - super::OFFSET as f32;

		let panel = Vec2::new(x_offset, y);

		OpponentPokemonGui {

			alive: false,

			pos: panel,

			orig_x: x,

			panel: byte_texture(include_bytes!("../../../assets/gui/opponent_pokemon.png")),			
			name: String::from("Opponent"),
			level: String::from("Lv"),
			health_bar: HealthBar::new(Vec2::new(39.0, 17.0), panel),

		}

	}

}

impl Entity for OpponentPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.health_bar.spawn();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.health_bar.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for OpponentPokemonGui {

	fn reset(&mut self) {
		self.update_position(self.orig_x - super::OFFSET, self.pos.y);
	}

	fn update(&mut self, delta: f32) {
		if self.alive {
			self.health_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.alive {
			draw(self.panel, self.pos.x, self.pos.y);
            draw_text_left(0, &self.name, TextColor::Black, self.pos.x + 8.0, self.pos.y + 2.0);
            draw_text_right(0, &self.level, TextColor::Black, self.pos.x + 86.0, self.pos.y + 2.0);
			self.health_bar.render(0.0);
		}		
	}

	fn update_gui(&mut self, pokemon: &PokemonInstance, new_pokemon: bool) {
		self.name = pokemon.name();
		self.level = format!("Lv{}", pokemon.data.level);
		self.health_bar.update_bar(new_pokemon, pokemon.current_hp, pokemon.base.hp);
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.pos.x = x;
		self.pos.y = y;
		self.health_bar.panel.x = x;
		self.health_bar.panel.y = y;
	}

	fn offset_position(&mut self, x: f32, y: f32) {
		self.update_position(self.pos.x + x, self.pos.y + y);
	}

}