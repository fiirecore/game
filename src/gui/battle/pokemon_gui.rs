use crate::entity::entity::Entity;
use crate::battle::battle::Battle;
use crate::io::data::Direction;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::text::TextRenderer;

use crate::gui::gui::{BasicText, Panel};
use crate::gui::battle::health_bar::HealthBar;
use crate::gui::gui::GuiComponent;

static OFFSET: isize = 24 * 5;
pub struct PlayerPokemonGui {

	alive: bool,

	orig_x: isize,

	pub panel: Panel,
	pub name: BasicText,
	pub level: BasicText,
	pub health_text: BasicText,
	pub health_bar: HealthBar,

}

impl PlayerPokemonGui {

	pub fn new(x: isize, y: isize) -> PlayerPokemonGui {

		let ppp_x = x + OFFSET;

		PlayerPokemonGui {

			alive: false,

			orig_x: ppp_x,

			panel: Panel::new("gui/battle/player_pokemon.png", ppp_x, y),
			name: BasicText::new(vec![String::from("Player")], 0, Direction::Left, 17, 2, ppp_x, y),
			level: BasicText::new(vec![String::from("Lv")], 0, Direction::Right, 95, 2, ppp_x, y),
			health_text: BasicText::new(vec![String::from("/")], 0, Direction::Right, 95, 20, ppp_x, y),
			health_bar: HealthBar::new(48, 17, ppp_x, y),

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
		self.update_position(self.orig_x, self.panel.y);
	}

	fn update(&mut self) {
		if self.is_alive() {
			self.health_bar.update();
		}		
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_alive() {
			self.panel.render(ctx, g, tr);
			self.name.render(ctx, g, tr);
			self.level.render(ctx, g, tr);
			self.health_text.render(ctx, g, tr);
			self.health_bar.render(ctx, g, tr);
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.player().pokemon.data.name.to_uppercase()];
		let mut plstr = String::from("Lv");
		plstr.push_str(battle.player().level.to_string().as_str());
		self.level.text = vec![plstr];
		self.update_hp(battle.player().current_hp, battle.player().base.hp);
	}

	fn update_hp(&mut self, current_health: u16, max_health: u16)  {
		self.health_bar.update_bar(current_health, max_health);
		let mut ch = current_health.to_string();
		ch.push('/');
		ch.push_str(max_health.to_string().as_str());
		self.health_text.text = vec![ch];
	}

	fn health_bar(&mut self) -> &mut HealthBar {
		&mut self.health_bar
	}

	fn update_position(&mut self, x: isize, y: isize) {
		self.panel.update_position(x, y);
		self.name.update_position(x, y);
		self.level.update_position(x, y);
		self.health_text.update_position(x , y);
		self.health_bar.update_position(x, y);
	}

	fn offset_position(&mut self, x: isize, y: isize) {
		self.update_position(self.panel.x + x, self.panel.y + y);
	}

}

pub struct OpponentPokemonGui {

	alive: bool,

	orig_x: isize,

	pub panel: Panel,
	pub name: BasicText,
	pub level: BasicText,
	pub health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(x: isize, y: isize) -> OpponentPokemonGui {

		let x = x - OFFSET;

		OpponentPokemonGui {

			alive: false,

			orig_x: x,

			panel: Panel::new("gui/battle/opponent_pokemon.png", x, y),			
			name: BasicText::new(vec![String::from("Opponent")], 0, Direction::Left, 8, 2, x, y),
			level: BasicText::new(vec![String::from("Lv")], 0, Direction::Right, 86, 2, x, y),
			health_bar: HealthBar::new(39, 17, x, y),

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
		self.update_position(self.orig_x, self.panel.y);
	}

	fn update(&mut self) {
		if self.is_alive() {
			self.health_bar.update();
		}		
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_alive() {
			self.panel.render(ctx, g, tr);
			self.name.render(ctx, g, tr);
			self.level.render(ctx, g, tr);
			self.health_bar.render(ctx, g, tr);
		}		
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.name.text = vec![battle.opponent().pokemon.data.name.to_uppercase()];
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

	fn update_position(&mut self, x: isize, y: isize) {
		self.panel.update_position(x, y);
		self.name.update_position(x, y);
		self.level.update_position(x, y);
		self.health_bar.update_position(x, y);
	}

	fn offset_position(&mut self, x: isize, y: isize) {
		self.update_position(self.panel.x + x, self.panel.y + y);
	}

}

pub trait PokemonGui: Entity { // To-do: sort out trait or have it extend something

	fn reset(&mut self);

	fn update(&mut self);

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer);

	fn update_gui(&mut self, battle: &Battle);

	fn update_hp(&mut self, current_hp: u16, max_hp: u16);

	fn health_bar(&mut self) -> &mut HealthBar;

	fn update_position(&mut self, x: isize, y: isize);

	fn offset_position(&mut self, x: isize, y: isize);

}