use crate::entity::util::direction::Direction;
use crate::game::battle::battle::Battle;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::text::TextRenderer;

use crate::gui::gui::{BasicText, Panel};
use crate::gui::battle::health_bar::HealthBar;
use crate::gui::gui::GuiComponent;
pub struct PlayerPokemonGui {

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
	

}

impl PokemonGui for PlayerPokemonGui {

	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		self.panel.render(ctx, g, tr);
		self.name.render(ctx, g, tr);
		self.level.render(ctx, g, tr);
		self.health_text.render(ctx, g, tr);
		self.health_bar.render(ctx, g, tr);
	}

	fn update_gui(&mut self, battle: &Battle) {
		self.health_bar.update_bar(battle.player_pokemon.current_hp, battle.player_pokemon.hp);
		self.health_text.text = String::from(battle.player_pokemon.current_hp.to_string().as_str());
		self.health_text.text.push('/');
		self.health_text.text.push_str(battle.player_pokemon.hp.to_string().as_str());
	}

}

pub struct OpponentPokemonGui {

	pub panel: Panel,
	pub name: BasicText,
	pub level: BasicText,
	pub health_bar: HealthBar,

}

impl OpponentPokemonGui {

	pub fn new(opp_x: isize, opp_y: isize) -> OpponentPokemonGui {
		OpponentPokemonGui {

			panel: Panel::new("gui/battle/opponent_pokemon.png", opp_x, opp_y),			
			name: BasicText::new("Opponent", 0, Direction::Left, 8, 2, opp_x, opp_y),
			level: BasicText::new("Lv", 0, Direction::Right, 86, 2, opp_x, opp_y),
			health_bar: HealthBar::new(39, 17, opp_x, opp_y),

		}
	}

}

impl PokemonGui for OpponentPokemonGui {

	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {

		self.panel.render(ctx, g, tr);
		self.name.render(ctx, g, tr);
		self.level.render(ctx, g, tr);
		self.health_bar.render(ctx, g, tr);

	}

	fn update_gui(&mut self, battle: &Battle) {
		self.health_bar.update_bar(battle.opponent_pokemon.current_hp, battle.opponent_pokemon.hp);
	}

}

pub trait PokemonGui {

	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer);

	fn update_gui(&mut self, battle: &Battle);

}