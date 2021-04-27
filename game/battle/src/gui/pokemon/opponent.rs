use game::{
	util::{Entity, Reset, text::TextColor},
	pokedex::pokemon::instance::PokemonInstance,
	macroquad::prelude::{Vec2, Texture2D},
	gui::health::HealthBar,
	graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

use super::{PokemonGui, PokemonGuiOffset};

pub struct OpponentPokemonGui {

	alive: bool,

	origin: Vec2,
	offset: PokemonGuiOffset,

	panel: Texture2D,
	name: String,
	level: String,
	health: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(origin: Vec2) -> OpponentPokemonGui {

		OpponentPokemonGui {

			alive: false,

			origin,
			offset: PokemonGuiOffset::new(),

			panel: byte_texture(include_bytes!("../../../assets/gui/opponent_pokemon.png")),			
			name: String::new(),
			level: String::new(),
			health: HealthBar::new(origin + Vec2::new(39.0, super::HEALTH_Y_OFFSET), Vec2::new(-super::OFFSET, 0.0)),

		}

	}

}

impl Entity for OpponentPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.health.spawn();
		self.offset.reset();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.health.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for OpponentPokemonGui {

	fn update(&mut self, delta: f32) {
		if self.alive {
			self.health.update(delta);
		}		
	}

	fn render(&self) {
		if self.alive {
			let x = self.origin.x - self.offset.x;
			draw(self.panel, x, self.origin.y);
			let y = self.origin.y + 2.0;
            draw_text_left(0, &self.name, TextColor::Black, x + 8.0, y);
            draw_text_right(0, &self.level, TextColor::Black, x + 86.0, y);
			self.health.render();
		}		
	}

	fn update_gui(&mut self, pokemon: &PokemonInstance, new_pokemon: bool) {
		self.name = pokemon.name();
		self.level = format!("Lv{}", pokemon.data.level);
		self.health.update_bar(new_pokemon, pokemon.current_hp, pokemon.base.hp);
	}

	fn offset(&mut self, delta: f32) -> bool {
		if self.alive {
			let done = self.offset.update(delta);
			self.health.offset.x = -self.offset.x;
			done
		} else {
			false
		}
	}

}