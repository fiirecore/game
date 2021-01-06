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
	
	pub text: Vec<String>,

	font_id: usize,
	counter: u16,
	
	pub no_pause: bool,
	pub can_continue: bool,

	pub next: u8,

	button_pos: i8,
	button_dir: i8,

}

impl IntroText {
	
	pub fn new(_panel_x: isize, _panel_y: isize, text: Vec<String>) -> IntroText {
		
		IntroText {

			alive: false,
			focus: false,

			x: 11,
			y: 11,
			panel_x: _panel_x,
			panel_y: _panel_y,

			text: text,

			font_id: 1,
			counter: 0,

			can_continue: false,
			no_pause: false,
			next: 0,
			
			button_pos: 0,
			button_dir: 1,

		}
		
	}

	fn reset(&mut self) {
		self.no_pause = false;
		self.counter = 0;
		self.can_continue = false;
		self.button_pos = 0;
		self.button_dir = 1;
	}

}

impl GuiComponent for IntroText {
	
	fn enable(&mut self) {
		self.next = 0;
		self.focus = true;
		self.alive = true;	
		self.reset();
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
				if self.button_pos % (4*8) == 0 {
					self.button_dir *= -1;
				}
				self.button_pos += self.button_dir;
			} else if self.counter as usize <= self.text[self.next as usize].len() * 4 {
				self.counter+=1;
			} else {
				self.counter = self.text[self.next as usize].len() as u16 * 4;
				self.can_continue = true;
			}
		}
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_active() {
			let mut string = String::new();
			let mut count = 0;
			for character in self.text[self.next as usize].chars() {
				if count >= self.counter / 4 {
					break;
				}
				string.push(character);
				count+=1;
			}
			tr.render_text_from_left(ctx, g, self.font_id, string.as_str(), self.panel_x + self.x, self.panel_y + self.y);
			if self.can_continue && self.text.len() as u8 - 1 != self.next {
				tr.render_button(ctx, g, self.get_text(), self.font_id, self.button_pos / 8, self.panel_x + self.x, self.panel_y + self.y /*- 2*/);
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
		self.text[self.next as usize].as_str()
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
		if self.can_continue {
			if context.keys[0] == 1 && !self.no_pause {
				self.reset();
				self.next += 1;
			}
		}
	}

	fn next(&self) -> u8 {
		self.next
	}

}