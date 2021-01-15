use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

//use crate::battle::battles::test_battle::TestBattle;

use crate::entity::entity::Entity;
use crate::gui::gui::{GuiComponent, Activatable, GuiText};
use crate::util::timer::Timer;
//use crate::gui::battle::battle_gui::BattleGuiComponent;

pub struct IntroText { // and outro

	alive: bool,
    focus: bool,
    
	x: isize,
	y: isize,
	panel_x: isize,
	panel_y: isize,
	
	pub text: Vec<Vec<String>>,
	current_phrase: u8,
	current_line: usize,
	font_id: usize,
	counter: u16,
	
	pub no_pause: bool,
	pub can_continue: bool,
	pub timer: Timer,

	button_pos: i8,
	button_dir: i8,

}

impl IntroText {
	
	pub fn new(_panel_x: isize, _panel_y: isize, text: Vec<Vec<String>>) -> IntroText {
		
		IntroText {

			alive: false,
			focus: false,

			x: 11,
			y: 11,
			panel_x: _panel_x,
			panel_y: _panel_y,

			text: text,
			current_phrase: 0,
			current_line: 0,

			font_id: 1,
			counter: 0,

			can_continue: false,
			no_pause: false,
			timer: Timer::new(60),
			
			button_pos: 0,
			button_dir: 1,

		}
		
	}

	fn reset(&mut self) {
		self.current_phrase = 0;
		self.button_pos = 0;
		self.button_dir = 1;
		self.reset_phrase();
	}

	fn reset_phrase(&mut self) {
		self.can_continue = false;
		self.no_pause = self.current_phrase >= 1;
		self.current_line = 0;
		self.counter = 0;
	}

}

impl GuiComponent for IntroText {
	
	fn enable(&mut self) {
		self.focus = true;
		self.alive = true;	
		self.reset();
	}
	
	fn disable(&mut self) {
		self.focus = false;
		self.alive = false;		
        self.timer.despawn();
		self.reset();
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, _context: &mut GameContext) {
		if self.is_active() {
			let line_len = self.get_line(self.current_line).len() as u16 * 4;
			if self.can_continue {
				if self.no_pause {
					if !self.timer.is_alive() {
						self.timer.spawn();
					}
					self.timer.update();
					if self.timer.is_finished() {
						if self.next() + 1 != self.text.len() as u8 {
							self.current_phrase += 1;
							self.reset_phrase();
							self.timer.reset();
							self.timer.despawn();
						}
					}
				}
				if self.button_pos % (4*8) == 0 {
					self.button_dir *= -1;
				}
				self.button_pos += self.button_dir;
			} else if self.counter <= line_len {
				self.counter += 1;
			} else if self.current_line < self.get_text().len() - 1 {
				self.current_line += 1;
				self.counter = 0;
			} else {
				self.counter = line_len;
				self.can_continue = true;
			}
		}
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_active() {
			let mut string = String::new();
			let mut count = 0;

			for character in self.get_line(self.current_line).chars() {
				if count >= self.counter / 4 {
					break;
				}
				string.push(character);
				count+=1;
			}

			tr.render_text_from_left(ctx, g, self.font_id, string.as_str(), self.panel_x + self.x, self.panel_y + self.y + self.current_line as isize * 16);

			for line_index in 0..self.current_line {
				tr.render_text_from_left(ctx, g, self.font_id, self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + line_index as isize * 16);
			}

			if self.can_continue && !self.no_pause {
				tr.render_button(ctx, g, self.get_line(self.get_text().len() - 1), self.font_id, self.button_pos / 8, self.panel_x + self.x, self.panel_y + self.y + self.current_line as isize * 16/*- 2*/);
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

	fn get_line(&self, index: usize) -> &String {
		&self.get_text()[index]
	}
	
	fn get_text(&self) -> &Vec<String> {
		&self.text[self.current_phrase as usize]
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
    }

    fn in_focus(&mut self) -> bool {
        self.focus
    }

	fn input(&mut self, context: &mut GameContext) {
		if self.can_continue {
			if context.keys[0] == 1 && !self.no_pause {
				self.current_phrase += 1;
				self.reset_phrase();
			}
		}
	}

	fn next(&self) -> u8 {
		self.current_phrase
	}

}