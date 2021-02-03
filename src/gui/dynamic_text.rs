use crate::io::data::text::Message;
use crate::util::Completable;
use crate::util::Reset;
use crate::util::graphics::draw_text_left_color;
use crate::util::input;

use crate::entity::Entity;
use crate::gui::{GuiComponent, Activatable};
use crate::util::timer::Timer;

pub struct DynamicText {

	alive: bool,
    focus: bool,
    
	x: f32,
	y: f32,
	panel_x: f32,
	panel_y: f32,
	
	pub text: Vec<Message>,
	current_phrase: u8,
	current_line: usize,
	counter: f32,
	
	pub can_continue: bool,
	finish_click: bool,
	pub timer: Timer,

	button_pos: f32,
	button_up: bool,

}

impl Default for DynamicText {
    fn default() -> Self {
        Self {
			alive: false,
			focus: false,

			x: 0.0,
			y: 0.0,
			panel_x: 0.0,
			panel_y: 0.0,
			text: Vec::new(),
			current_phrase: 0,
			current_line: 0,

			counter: 0.0,

			can_continue: false,
			finish_click: false,
			timer: Timer::new(1.0),
			
			button_pos: 0.0,
			button_up: true,
		}
    }
}

impl DynamicText {
	
	pub fn new(text_x: f32, text_y: f32, panel_x: f32, panel_y: f32) -> Self {
		
		Self {
			x: text_x,
			y: text_y,
			panel_x: panel_x,
			panel_y: panel_y,
			..Default::default()
		}
		
	}

	fn reset_phrase(&mut self) {
		self.can_continue = false;
		//self.no_pause = self.current_phrase >= 1;
		self.current_line = 0;
		self.counter = 0.0;
	}

	fn current_line(&self) -> &String {
		&self.current_message().message[self.current_line]
	}

	fn current_message(&self) -> &Message {
		&self.text[self.current_phrase as usize]
	}

}

impl GuiComponent for DynamicText {
	
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
			let line_len = (self.current_line().len() as u16) << 2;
			if self.can_continue {
				if self.current_message().no_pause {
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
				self.counter += delta * 60.0 * if macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::Space) {
					16.0
				} else {
					1.0
				}
			} else if self.current_line < self.text[self.current_phrase as usize].message.len() - 1 {
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

			let current_line = self.current_line();

			let string = if current_line.len() > (self.counter / 4.0) as usize {
				&current_line[..(self.counter / 4.0) as usize]
			} else {
				current_line
			};		

			draw_text_left_color(self.current_message().font_id, string, self.current_message().color, self.panel_x + self.x, self.panel_y + self.y + (self.current_line << 4) as f32);

			for line_index in 0..self.current_line {
				draw_text_left_color(self.current_message().font_id, &self.current_message().message[line_index], self.current_message().color, self.panel_x + self.x, self.panel_y + self.y + (line_index << 4) as f32);
			}

			if self.can_continue && !self.current_message().no_pause {
				crate::util::graphics::draw_button(current_line, self.current_message().font_id, self.panel_x + self.x, self.panel_y + self.y + self.button_pos + (self.current_line << 4) as f32);
			}			
		
		}
		
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel_x = x as f32;
		self.panel_y = y as f32;
	}
	
}

impl Activatable for DynamicText {

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
		if self.can_continue && self.focus {
			if input::pressed(crate::util::input::Control::A) && !self.current_message().no_pause {
				if self.next() + 1 == self.text.len() as u8 {
					self.finish_click = true;
				} else {
					self.current_phrase += 1;
					self.reset_phrase();
				}
			}
		}
	}

	fn next(&self) -> u8 {
		self.current_phrase
	}

}

impl Reset for DynamicText {
    fn reset(&mut self) {
        self.current_phrase = 0;
		self.button_pos = 0.0;
		self.button_up = true;
		self.finish_click = false;
		self.reset_phrase();
    }
}

impl Completable for DynamicText {
    fn is_finished(&self) -> bool {
		self.can_continue && 
		self.next() + 1 == self.text.len() as u8 &&
		if self.current_message().no_pause {
			self.timer.is_finished()
		} else {
			self.finish_click
		}
    }
}