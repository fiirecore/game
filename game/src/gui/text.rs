use firecore_util::{Entity, Timer, Reset, Completable, text::Message};
use firecore_input as input;

use macroquad::prelude::Vec2;

use crate::graphics::{draw_text_left, draw_button};

#[derive(Clone)]
pub struct DynamicText {

	alive: bool,
    
	pos: Vec2,
	panel: Vec2,
	
	pub messages: Option<Vec<Message>>,
	current_message: usize,
	current_line: usize,

	counter: f32,
	
	pub can_continue: bool,
	finish_click: bool,
	timer: Timer,

	button_pos: f32,
	button_up: bool,

}

impl DynamicText {

	pub fn with_size(size: usize, pos: Vec2, panel: Vec2) -> Self {
		Self {
			messages: Some(Vec::with_capacity(size)),
			..Self::new(pos, panel)
		}
	}

	pub fn from_text(pos: Vec2, panel: Vec2, messages: Vec<Message>) -> Self {
		Self {
			messages: Some(messages),
			..Self::new(pos, panel)
		}
	}
	
	pub fn new(pos: Vec2, panel: Vec2) -> Self {
		Self {

			alive: false,

			pos,
			panel,

			messages: None,
			current_message: 0,
			current_line: 0,

			counter: 0.0,

			can_continue: false,
			finish_click: false,
			timer: Timer::new(1.0),
			
			button_pos: 0.0,
			button_up: true,
		}
	}

	fn reset_message(&mut self) {
		self.can_continue = false;
		self.current_line = 0;
		self.counter = 0.0;
		self.timer.hard_reset();
	}

	pub fn input(&mut self) {
		if self.can_continue {
			if let Some(messages) = self.messages.as_ref() {
				if input::pressed(input::Control::A) && messages[self.current_message].wait.is_none() {
					if self.current_message + 1 >= messages.len() {
						self.finish_click = true;
					} else {
						self.current_message += 1;
						self.reset_message();
					}	
				}				
			}
			
		}
	}

	pub fn update(&mut self, delta: f32) {
		if self.alive {
			if let Some(messages) = self.messages.as_ref() {
				let current = &messages[self.current_message];
				let line_len = current.lines[self.current_line].len() << 2;
				if self.can_continue {
					
					if let Some(time) = current.wait {
						if !self.timer.is_alive() {
							self.timer.spawn();
							self.timer.set_target(time);
						}
						self.timer.update(delta);
						if self.timer.is_finished() {
							if self.current_message + 1 != messages.len() {
								self.current_message += 1;
								self.reset_message();
								self.timer.soft_reset();
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
					self.counter += delta * 60.0
				} else if self.current_line < current.lines.len() - 1 {
					self.current_line += 1;
					self.counter = 0.0;
				} else {
					self.counter = line_len as f32;
					self.can_continue = true;
				}
			}			
		}
	}

	pub fn render(&self) {
		if self.alive {
			if let Some(messages) = self.messages.as_ref() {
				let current_line = &messages[self.current_message].lines[self.current_line];

				let string = if current_line.len() > (self.counter as usize) >> 2 {
					&current_line[..(self.counter as usize) >> 2]
				} else {
					current_line
				};

				let current = &messages[self.current_message];

				draw_text_left(current.font_id, string, current.color, self.panel.x + self.pos.x, self.panel.y + self.pos.y + (self.current_line << 4) as f32);

				for index in 0..self.current_line {
					draw_text_left(current.font_id, &current.lines[index], current.color, self.panel.x + self.pos.x, self.panel.y + self.pos.y + (index << 4) as f32);
				}

				if self.can_continue && current.wait.is_none() {
					draw_button(current_line, current.font_id, self.panel.x + self.pos.x, self.panel.y + self.pos.y + self.button_pos + (self.current_line << 4) as f32);
				}		

			}

		}
		
	}

	pub fn current_message(&self) -> usize {
		self.current_message
	}

}

impl Reset for DynamicText {
    fn reset(&mut self) {
        self.current_message = 0;
		self.button_pos = 0.0;
		self.button_up = true;
		self.finish_click = false;
		self.reset_message();
    }
}

impl Completable for DynamicText {
    fn is_finished(&self) -> bool {
		if let Some(messages) = self.messages.as_ref() {
			self.current_message + 1 == messages.len() && if messages[self.current_message].wait.is_none() {
				self.finish_click
			} else {
				self.timer.is_finished()
			} &&
			self.can_continue
		} else {
			false
		}		
    }
}

impl Entity for DynamicText {

	fn spawn(&mut self) {
		self.alive = true;	
		self.reset();
	}
	
	fn despawn(&mut self) {
		self.alive = false;		
        self.timer.despawn();
		self.reset();
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}

}