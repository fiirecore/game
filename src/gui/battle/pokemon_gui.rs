use crate::entity::entity::Entity;
use crate::entity::util::direction::Direction;
use crate::game::battle::battle::Battle;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::text::TextRenderer;

use crate::gui::gui::{BasicText, Panel};
use crate::gui::battle::health_bar::HealthBar;
use crate::gui::gui::GuiComponent;
pub struct PlayerPokemonGui {

	alive: bool,

	x: isize,
	y: isize,

	pub panel: Panel,
	pub name: BasicText,
	pub level: BasicText,
	pub health_text: BasicText,
	pub health_bar: HealthBar,

}

impl PlayerPokemonGui {

	pub fn new(ppp_x: isize, ppp_y: isize) -> PlayerPokemonGui {
		PlayerPokemonGui {

			alive: false,

			x: ppp_x,
			y: ppp_y,

			panel: Panel::new("gui/battle/player_pokemon.png", ppp_x, ppp_y),
			name: BasicText::new("Player", 0, Direction::Left, 17, 2, ppp_x, ppp_y),
			level: BasicText::new("Lv", 0, Direction::Right, 95, 2, ppp_x, ppp_y),
			health_text: BasicText::new("/", 0, Direction::Right, 95, 20, ppp_x, ppp_y),
			health_bar: HealthBar::new(48, 17, ppp_x, ppp_y),

		}
	}

	pub fn offset_position(&mut self, x: isize, y: isize) {
		self.panel.update_position(self.x + x, self.y + y);
		self.name.update_position(self.x + x, self.y + y);
		self.level.update_position(self.x + x, self.y + y);
		self.health_text.update_position(self.x + x, self.y + y);
		self.health_bar.update_position(self.x + x, self.y + y);
	}	

	pub fn update_hp(&mut self, current_health: u16, max_health: u16)  {
		self.health_bar.update_bar(current_health, max_health);
		let mut ch = current_health.to_string();
		ch.push('/');
		ch.push_str(max_health.to_string().as_str());
		self.health_text.text = ch;
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
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.panel.disable();
		self.name.disable();
		self.level.disable();
		self.health_text.disable();
		self.health_bar.disable();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PokemonGui for PlayerPokemonGui {

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
		self.name.text = battle.player().pokemon.name.to_uppercase();
		let mut plstr = String::from("Lv");
		plstr.push_str(battle.player().level.to_string().as_str());
		self.level.text = plstr;
		self.update_hp(battle.player().current_hp, battle.player().base.hp);
	}

	fn freeze(&self) -> bool {
		self.health_bar.is_moving()
	}

}

pub struct OpponentPokemonGui {

	alive: bool,

	pub panel: Panel,
	pub name: BasicText,
	pub level: BasicText,
	pub health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(opp_x: isize, opp_y: isize) -> OpponentPokemonGui {

		OpponentPokemonGui {

			alive: false,

			panel: Panel::new("gui/battle/opponent_pokemon.png", opp_x, opp_y),			
			name: BasicText::new("Opponent", 0, Direction::Left, 8, 2, opp_x, opp_y),
			level: BasicText::new("Lv", 0, Direction::Right, 86, 2, opp_x, opp_y),
			health_bar: HealthBar::new(39, 17, opp_x, opp_y),

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
		self.name.text = battle.opponent().pokemon.name.to_uppercase();
		let mut olstr = String::from("Lv");
		olstr.push_str(battle.opponent().level.to_string().as_str());
		self.level.text = olstr;
		self.health_bar.update_bar(battle.opponent().current_hp, battle.opponent().base.hp);
	}

	fn freeze(&self) -> bool {
		self.health_bar.is_moving()
	}

}

pub trait PokemonGui { // To-do: sort out trait or have it extend something

	fn update(&mut self);

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer);

	fn update_gui(&mut self, battle: &Battle);

	fn freeze(&self) -> bool;

}