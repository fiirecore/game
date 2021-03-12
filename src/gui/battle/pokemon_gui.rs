use macroquad::prelude::Color;

use firecore_util::Entity;
use crate::battle::battle::Battle;
use crate::gui::background::Background;
use crate::gui::text::StaticText;
use firecore_util::text::TextColor;
use crate::gui::battle::health_bar::HealthBar;
use crate::gui::GuiComponent;
use crate::util::graphics::draw_rect;
use crate::util::graphics::texture::byte_texture;

static OFFSET: f32 = 24.0 * 5.0;
pub struct PlayerPokemonGui {

	alive: bool,

	pub orig_x: f32,

	pub panel: Background,
	pub name: StaticText,
	pub level: StaticText,
	pub health_text: StaticText,
	pub health_bar: HealthBar,
	pub exp_width: f32,

}

impl PlayerPokemonGui {

	const EXP_COLOR: Color = macroquad::color_u8!(64, 200, 248, 255);

	pub fn new(x: f32, y: f32) -> PlayerPokemonGui {

		let ppp_x = x + OFFSET;

		PlayerPokemonGui {

			alive: false,

			orig_x: x,

			panel: Background::new(byte_texture(include_bytes!("../../../build/assets/gui/battle/player_pokemon.png")), ppp_x, y),
			name: StaticText::new(vec![String::from("Player")], TextColor::Black, 0, false, 17.0, 2.0, ppp_x, y),
			level: StaticText::new(vec![String::from("Lv")], TextColor::Black, 0, true, 95.0, 2.0, ppp_x, y),
			health_text: StaticText::new(vec![String::from("/")], TextColor::Black, 0, true, 95.0, 20.0, ppp_x, y),
			health_bar: HealthBar::new(48.0, 17.0, ppp_x, y),
			exp_width: 0.0,

		}
	}

}

impl Entity for PlayerPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.panel.spawn();
		self.name.spawn();
		self.level.spawn();
		self.health_text.spawn();
		self.health_bar.spawn();
		self.health_bar.reset();
		self.reset();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.panel.despawn();
		self.name.despawn();
		self.level.despawn();
		self.health_text.despawn();
		self.health_bar.despawn();
		self.reset();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for PlayerPokemonGui {

	fn reset(&mut self) {
		self.update_position(self.orig_x + OFFSET, self.panel.y);
	}

	fn update(&mut self, delta: f32) {
		if self.is_alive() {
			self.health_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.is_alive() {
			self.panel.render();
			self.name.render();
			self.level.render();
			self.health_text.render();
			self.health_bar.render();
			draw_rect(Self::EXP_COLOR, self.panel.x + 32.0, self.panel.y + 33.0, (self.exp_width * 64.0) as u32, 2);
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.player().pokemon.data.name.to_ascii_uppercase()];
		self.level.text = vec![String::from("Lv") + battle.player().level.to_string().as_str()];
		self.exp_width = battle.player().exp as f32 / battle.player().pokemon.training.growth_rate.level_exp(battle.player().level) as f32;
		self.update_hp(battle.player().current_hp, battle.player().base.hp);
	}

	fn update_hp(&mut self, current_health: u16, max_health: u16)  {
		self.health_bar.update_bar(current_health, max_health);
		self.health_text.text = vec![current_health.to_string() + "/" + max_health.to_string().as_str()];
	}

	fn health_bar(&mut self) -> &mut HealthBar {
		&mut self.health_bar
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel.update_position(x, y);
		self.name.update_position(x, y);
		self.level.update_position(x, y);
		self.health_text.update_position(x , y);
		self.health_bar.update_position(x, y);
	}

	fn offset_position(&mut self, x: f32, y: f32) {
		self.update_position(self.panel.x + x, self.panel.y + y);
	}

}

pub struct OpponentPokemonGui {

	alive: bool,

	pub orig_x: f32,

	pub panel: Background,
	pub name: StaticText,
	pub level: StaticText,
	pub health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(x: f32, y: f32) -> OpponentPokemonGui {

		let x_offset = x - OFFSET as f32;

		OpponentPokemonGui {

			alive: false,

			orig_x: x,

			panel: Background::new(byte_texture(include_bytes!("../../../build/assets/gui/battle/opponent_pokemon.png")), x_offset, y),			
			name: StaticText::new(vec![String::from("Opponent")], TextColor::Black, 0, false, 8.0, 2.0, x_offset, y),
			level: StaticText::new(vec![String::from("Lv")], TextColor::Black, 0, true, 86.0, 2.0, x_offset, y),
			health_bar: HealthBar::new(39.0, 17.0, x_offset, y),

		}

	}

}

impl Entity for OpponentPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.panel.spawn();
		self.name.spawn();
		self.level.spawn();
		self.health_bar.spawn();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.panel.despawn();
		self.name.despawn();
		self.level.despawn();
		self.health_bar.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for OpponentPokemonGui {

	fn reset(&mut self) {
		self.update_position(self.orig_x - OFFSET, self.panel.y);
	}

	fn update(&mut self, delta: f32) {
		if self.is_alive() {
			self.health_bar.update(delta);
		}		
	}

	fn render(&self) {
		if self.is_alive() {
			self.panel.render();
			self.name.render();
			self.level.render();
			self.health_bar.render();
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.opponent().pokemon.data.name.to_ascii_uppercase()];
		self.level.text = vec![String::from("Lv") + battle.opponent().level.to_string().as_str()];
		self.update_hp(battle.opponent().current_hp, battle.opponent().base.hp)
	}

	fn update_hp(&mut self, current_hp: u16, max_hp: u16) {
		self.health_bar.update_bar(current_hp, max_hp);
	}

	fn health_bar(&mut self) -> &mut HealthBar {
		&mut self.health_bar
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel.update_position(x, y);
		self.name.update_position(x, y);
		self.level.update_position(x, y);
		self.health_bar.update_position(x, y);
	}

	fn offset_position(&mut self, x: f32, y: f32) {
		self.update_position(self.panel.x + x, self.panel.y + y);
	}

}

pub trait PokemonGui: Entity { // To-do: sort out trait or have it extend something

	fn reset(&mut self);

	fn update(&mut self, delta: f32);

	fn render(&self);

	fn update_gui(&mut self, battle: &Battle);

	fn update_hp(&mut self, current_hp: u16, max_hp: u16);

	fn health_bar(&mut self) -> &mut HealthBar;

	fn update_position(&mut self, x: f32, y: f32);

	fn offset_position(&mut self, x: f32, y: f32);

}