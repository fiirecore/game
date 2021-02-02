use crate::util::input;

use crate::entity::Entity;
use crate::gui::{GuiComponent, Activatable, GuiText};
use crate::util::render::draw_text_left;
use crate::util::timer::Timer;

pub struct IntroText { // and outro

	alive: bool,
    focus: bool,
    
	x: f32,
	y: f32,
	panel_x: f32,
	panel_y: f32,
	
	pub text: Vec<Vec<String>>,
	current_phrase: u8,
	current_line: usize,
	font_id: usize,
	counter: f32,
	
	pub no_pause: bool,
	pub can_continue: bool,
	pub timer: Timer,

	button_pos: f32,
	button_up: bool,

}

impl IntroText {
	
	pub fn new(panel_x: f32, panel_y: f32, text: Vec<Vec<String>>) -> IntroText {
		
		IntroText {

			alive: false,
			focus: false,

			x: 11.0,
			y: 11.0,
			panel_x: panel_x,
			panel_y: panel_y,

			text: text,
			current_phrase: 0,
			current_line: 0,

			font_id: 1,
			counter: 0.0,

			can_continue: false,
			no_pause: false,
			timer: Timer::new(1.0),
			
			button_pos: 0.0,
			button_up: true,

		}
		
	}

	fn reset(&mut self) {
		self.current_phrase = 0;
		self.button_pos = 0.0;
		self.button_up = true;
		self.reset_phrase();
	}

	fn reset_phrase(&mut self) {
		self.can_continue = false;
		self.no_pause = self.current_phrase >= 1;
		self.current_line = 0;
		self.counter = 0.0;
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

	fn update(&mut self, delta: f32) {
		if self.is_active() {
			let line_len = self.get_line(self.current_line).len() as u16 * 4;
			if self.can_continue {
				if self.no_pause {
					if !self.timer.is_alive() {
						self.timer.spawn();
					}
					self.timer.update(delta);
					if self.timer.is_finished() {
						if self.next() + 1 != self.text.len() as u8 {
							self.current_phrase += 1;
							self.reset_phrase();
							self.timer.reset();
							self.timer.despawn();
						}
					}
				}
				if self.button_up {
					self.button_pos += delta * 7.5;
					if self.button_pos > 3.0 {
						self.button_up = !self.button_up;
					}
				} else {
					self.button_pos -= delta * 7.5;
					if self.button_pos < 0.0 {
						self.button_up = !self.button_up;
					}
				}
			} else if self.counter <= line_len as f32 {
				self.counter += delta * 60.0;
			} else if self.current_line < self.get_text().len() - 1 {
				self.current_line += 1;
				self.counter = 0.0;
			} else {
				self.counter = line_len as f32;
				self.can_continue = true;
			}
		}
	}

	fn render(&self) {
		if self.is_active() {
			let mut string = String::new();
			let mut count = 0;

			for character in self.get_line(self.current_line).chars() {
				if count >= (self.counter / 4.0) as i32 {
					break;
				}
				string.push(character);
				count+=1;
			}

			draw_text_left(self.font_id, string.as_str(), self.panel_x + self.x, self.panel_y + self.y + (self.current_line << 4) as f32);

			for line_index in 0..self.current_line {
				draw_text_left(self.font_id, self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + (line_index << 4) as f32);
			}

			if self.can_continue && !self.no_pause {
				crate::util::render::draw_button(self.get_line(self.get_text().len() - 1), self.font_id, self.panel_x + self.x, self.panel_y + self.y + self.button_pos + (self.current_line << 4) as f32);
			}			
		
		}
		
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel_x = x as f32;
		self.panel_y = y as f32;
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

	fn input(&mut self, _delta: f32) {
		if self.can_continue {
			if input::pressed(crate::util::input::Control::A) && !self.no_pause {
				self.current_phrase += 1;
				self.reset_phrase();
			}
		}
	}

	fn next(&self) -> u8 {
		self.current_phrase
	}

}