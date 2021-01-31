use crate::entity::Entity;
use crate::battle::battle::Battle;
use crate::gui::background::Background;
use crate::gui::text::BasicText;
use crate::io::data::Direction;
use crate::util::text_renderer::TextRenderer;

use crate::gui::battle::health_bar::HealthBar;
use crate::gui::GuiComponent;
use crate::util::texture::byte_texture;

static OFFSET: f32 = 24.0 * 5.0;
pub struct PlayerPokemonGui {

	alive: bool,

	pub orig_x: f32,

	pub panel: Background,
	pub name: BasicText,
	pub level: BasicText,
	pub health_text: BasicText,
	pub health_bar: HealthBar,

}

impl PlayerPokemonGui {

	pub fn new(x: f32, y: f32) -> PlayerPokemonGui {

		let ppp_x = x + OFFSET;

		PlayerPokemonGui {

			alive: false,

			orig_x: x,

			panel: Background::new(byte_texture(include_bytes!("../../../build/assets/gui/battle/player_pokemon.png")), ppp_x, y),
			name: BasicText::new(vec![String::from("Player")], 0, Direction::Left, 17.0, 2.0, ppp_x, y),
			level: BasicText::new(vec![String::from("Lv")], 0, Direction::Right, 95.0, 2.0, ppp_x, y),
			health_text: BasicText::new(vec![String::from("/")], 0, Direction::Right, 95.0, 20.0, ppp_x, y),
			health_bar: HealthBar::new(48.0, 17.0, ppp_x, y),

		}
	}

}

impl Entity for PlayerPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.panel.enable();
		self.name.enable();
		self.level.enable();
		self.health_text.enable();
		self.health_bar.enable();
		self.health_bar.reset();
		self.reset();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.panel.disable();
		self.name.disable();
		self.level.disable();
		self.health_text.disable();
		self.health_bar.disable();
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

	fn render(&self, tr: &TextRenderer) {
		if self.is_alive() {
			self.panel.render(tr);
			self.name.render(tr);
			self.level.render(tr);
			self.health_text.render(tr);
			self.health_bar.render(tr);
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.player().data.name.to_uppercase()];
		self.level.text = vec![String::from("Lv") + battle.player().level.to_string().as_str()];
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
	pub name: BasicText,
	pub level: BasicText,
	pub health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(x: f32, y: f32) -> OpponentPokemonGui {

		let x_offset = x - OFFSET as f32;

		OpponentPokemonGui {

			alive: false,

			orig_x: x,

			panel: Background::new(byte_texture(include_bytes!("../../../build/assets/gui/battle/opponent_pokemon.png")), x_offset, y),			
			name: BasicText::new(vec![String::from("Opponent")], 0, Direction::Left, 8.0, 2.0, x_offset, y),
			level: BasicText::new(vec![String::from("Lv")], 0, Direction::Right, 86.0, 2.0, x_offset, y),
			health_bar: HealthBar::new(39.0, 17.0, x_offset, y),

		}

	}

}

impl Entity for OpponentPokemonGui {

    fn spawn(&mut self) {
		self.alive = true;
		self.panel.enable();
		self.name.enable();
		self.level.enable();
		self.health_bar.enable();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.panel.disable();
		self.name.disable();
		self.level.disable();
		self.health_bar.disable();
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

	fn render(&self, tr: &TextRenderer) {
		if self.is_alive() {
			self.panel.render(tr);
			self.name.render(tr);
			self.level.render(tr);
			self.health_bar.render(tr);
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.opponent().data.name.to_uppercase()];
		let mut olstr = String::from("Lv");
		olstr.push_str(battle.opponent().level.to_string().as_str());
		self.level.text = vec![olstr];
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

	fn render(&self, tr: &TextRenderer);

	fn update_gui(&mut self, battle: &Battle);

	fn update_hp(&mut self, current_hp: u16, max_hp: u16);

	fn health_bar(&mut self) -> &mut HealthBar;

	fn update_position(&mut self, x: f32, y: f32);

	fn offset_position(&mut self, x: f32, y: f32);

}