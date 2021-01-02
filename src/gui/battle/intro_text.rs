use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

//use crate::game::battle::battles::test_battle::TestBattle;

use crate::gui::gui::{GuiComponent, Activatable, GuiText};
//use crate::gui::battle::battle_gui::BattleGuiComponent;

pub struct IntroText { // and outro

	alive: bool,
    focus: bool,
    
	x: isize,
	y: isize,
	panel_x: isize,
	panel_y: isize,
	

	pub player_text: String,
	pub text: String,
	font_id: usize,
	counter: u16,
	
	pub no_pause: bool,
	pub can_continue: bool,
	next: u8,

	button_pos: i8,
	button_dir: i8,

}

impl IntroText {
	
	pub fn new(_panel_x: isize, _panel_y: isize) -> IntroText {
		
		IntroText {

			alive: false,
			focus: false,

			x: 11,
			y: 11,
			panel_x: _panel_x,
			panel_y: _panel_y,

			player_text: String::new(),
			text: String::from("Intro Text"),
			font_id: 1,
			counter: 0,

			can_continue: false,
			next: 0,
			no_pause: false,
			
			button_pos: 0,
			button_dir: 1,

		}
		
	}

	pub fn update_text(&mut self, text: String) {
		self.text = text;
	}
	
}

impl GuiComponent for IntroText {
	
	fn enable(&mut self) {
		self.counter = 0;
		self.can_continue = false;
		self.button_pos = 0;
		self.button_dir = 1;
		self.next = 0;
		self.no_pause = false;
		self.focus = true;
		self.alive = true;	
	}
	
	fn disable(&mut self) {
		self.focus = false;
		self.alive = false;
		self.next = 0;
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, _context: &mut GameContext) {
		if self.is_active() {
			if self.can_continue {
				if self.button_pos % (4*8) == 0 || self.button_pos == 0 {
					self.button_dir *= -1;
				}
				self.button_pos += self.button_dir;
			}
			if self.counter as usize <= self.text.len() * 4 && !self.can_continue {
				self.counter+=1;
			} else {
				self.counter = self.text.len() as u16 * 4;
				self.can_continue = true;
			}
			if self.can_continue && self.no_pause {
				self.next = 2;
			}
		}
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
			if self.can_continue && !self.no_pause {
				tr.render_button(ctx, g, self.text.as_str(), self.font_id, self.button_pos / 8, self.panel_x + self.x, self.panel_y + self.y /*- 2*/);
			}
		}
		
	}

	fn update_position(&mut self, x: isize, y: isize) {
		self.panel_x = x;
		self.panel_y = y;
	}
	
}

/*

impl BattleGuiComponent for IntroText {
	
	fn update_gui(&mut self, _battle_manager: &mut BattleManager) {
		
	}
	
}

*/

impl GuiText for IntroText {
	
	fn get_text(&self) -> &str {
		self.text.as_str()
	}

	fn get_font_id(&self) -> usize {
		self.font_id
	}
	
}

impl Activatable for IntroText {

    fn focus(&mut self) {
        self.focus = true;
    }

    fn unfocus(&mut self) {
		self.focus = false;
		self.next = 0;
    }

    fn in_focus(&mut self) -> bool {
        self.focus
    }

	fn input(&mut self, context: &mut GameContext) {
		if context.keys[0] == 1 {
			if self.can_continue {
				self.next = 1;
			}			
		}
	}

	fn next(&self) -> u8 {
		self.next
	}

}