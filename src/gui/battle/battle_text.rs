use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

//use crate::game::battle::battle_manager::BattleManager;

use crate::entity::entity::Entity;
use crate::gui::gui::{GuiComponent, GuiText};
use crate::util::timer::Timer;

//use crate::gui::battle::battle_gui::BattleGuiComponent;

pub struct BattleText {
	
	alive: bool,

	x: isize,
	y: isize,
	panel_x: isize,
	panel_y: isize,
	
	pub text: String,
	pub font_id: usize,

	counter: u16,

	pub can_continue: bool,

	pub timer: Timer,
	
}

impl BattleText {
	
	pub fn new(_panel_x: isize, _panel_y: isize) -> BattleText {
		
		BattleText {

			alive: false,

			x: 11,
			y: 11,
			panel_x: _panel_x,
			panel_y: _panel_y,

			text: String::from("null"),
			font_id: 1,
			counter: 0,

			can_continue: false,

			//next: 0,
			//phrase: 0,

			timer: Timer::new(60),

			//button_pos: 0,
			//button_dir: 1,

		}
		
	}

	pub fn update_text(&mut self, pokemon: &String, pmove: &String) { // seperate into two lines not just one
		self.text = pokemon.clone();
		self.text.push_str(" used ");
		self.text.push_str(pmove.as_str());
		self.text.push('!');
	}

	pub fn update_faint(&mut self, pokemon: &String) {
		self.text = pokemon.clone();
		self.text.push_str(" fainted!");
	}

	fn reset(&mut self) {
		self.counter = 0;
		//self.next = 0;
		//self.phrase = 0;
		self.can_continue = false;
		self.timer.despawn();
	}

	pub fn update(&mut self) {
		if self.is_active() {
			if !self.can_continue {
				if self.counter as usize <= self.text.len() * 4 {
					self.counter+=1;
				} else {
					self.counter = self.text.len() as u16 * 4;
					self.can_continue = true;
				}
			}
		}
	}
	
}

impl GuiComponent for BattleText {
	
	fn enable(&mut self) {
		self.alive = true;	
		self.reset();
	}
	
	fn disable(&mut self) {
		self.alive = false;
		self.timer.despawn();
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, _context: &mut GameContext) {
		println!("Wrong update method for battle text");
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_active() {
			let mut string = String::new();
			let mut count = 0;
			for character in self.text.chars() {
				if count >= self.counter / 4 {
					break;
				}
				string.push(character);
				count+=1;
			}
			tr.render_text_from_left(ctx, g, self.font_id, string.as_str(), self.panel_x + self.x, self.panel_y + self.y);
		}
	}

	fn update_position(&mut self, x: isize, y: isize) {
		self.panel_x = x;
		self.panel_y = y;
	}
	
}

impl GuiText for BattleText {
	
	fn get_text(&self) -> &str {
		self.text.as_str()
	}
	
	fn get_font_id(&self) -> usize {
		self.font_id
	}

}

/*

impl BattleActivatable for BattleText {

	fn focus(&mut self) {
		self.focus = true;
	}

	fn unfocus(&mut self) {
		self.focus = false;
	}

	fn in_focus(&mut self) -> bool {
		self.focus
	}

	fn input(&mut self, context: &mut GameContext, battle: &mut Battle, pp_gui: &mut PlayerPokemonGui, op_gui: &mut OpponentPokemonGui) {
		if context.keys[0] == 1 {
			if self.can_continue {
				if self.phrase == 0 {
					if battle.player().base.speed > battle.opponent().base.speed {
						op_gui.update_gui(battle);
					} else {
						pp_gui.update_gui(battle);
					}
					self.next = 1;
					self.phrase = 1;
					self.counter = 0;
					self.can_continue = false;
				} else if self.phrase == 1 {
					if battle.player().base.speed > battle.opponent().base.speed {
						pp_gui.update_gui(battle);
					} else {
						op_gui.update_gui(battle);
					}
					pp_gui.update_gui(battle);
					self.next = 2;
				}
				
			}			
		}
	}

	fn next(&self) -> u8 {
		self.next
	}

}

*/