use serde::Deserialize;

use crate::io::data::text::Message;
use crate::io::data::text::MessageSet;
use crate::util::Completable;
use crate::util::Reset;
use crate::util::graphics::draw_text_left_color;
use frc_input as input;

use crate::util::Entity;
use crate::gui::GuiComponent;
use crate::util::timer::Timer;

#[derive(Clone, Deserialize)]
pub struct DynamicText {

	#[serde(skip)]
	alive: bool,
	#[serde(skip)]
    focus: bool,
    
	#[serde(default = "dx")]
	x: f32,
	#[serde(default = "dy")]
	y: f32,
	#[serde(default = "px")]
	panel_x: f32,
	#[serde(default = "py")]
	panel_y: f32,
	
	pub text: MessageSet,
	#[serde(skip)]
	current_phrase: u8,
	#[serde(skip)]
	current_line: usize,
	#[serde(skip)]
	counter: f32,
	
	#[serde(skip)]
	pub can_continue: bool,
	#[serde(skip)]
	finish_click: bool,
	#[serde(skip)]
	pub timer: Timer,

	#[serde(skip)]
	button_pos: f32,
	#[serde(skip)]
	button_up: bool,

}

impl DynamicText {

	pub fn from_text(text_x: f32, text_y: f32, panel_x: f32, panel_y: f32, text: MessageSet) -> Self {
		Self {
			text,
			..Self::new(text_x, text_y, panel_x, panel_y)
		}
	}
	
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
		&self.text.get_phrase(self.current_phrase as usize)
	}

	pub fn current_phrase(&self) -> u8 {
		self.current_phrase
	}

}

impl GuiComponent for DynamicText {

	fn update(&mut self, delta: f32) {
		if self.is_alive() {
			let line_len = (self.current_line().len() as u16) << 2;
			if self.can_continue {
				if self.current_message().no_pause {
					if !self.timer.is_alive() {
						self.timer.spawn();
					}
					self.timer.update(delta);
					if self.timer.is_finished() {
						if self.current_phrase() + 1 != self.text.len() as u8 {
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
					8.0
				} else {
					1.0
				}
			} else if self.current_line < self.current_message().message.len() - 1 {
				self.current_line += 1;
				self.counter = 0.0;
			} else {
				self.counter = line_len as f32;
				self.can_continue = true;
			}
		}
	}

	fn render(&self) {
		if self.is_alive() {

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

impl super::Focus for DynamicText {

    fn focus(&mut self) {
        self.focus = true;
    }

    fn unfocus(&mut self) {
		self.focus = false;
    }

    fn in_focus(&mut self) -> bool {
        self.focus
    }

}

impl crate::util::Input for DynamicText {

	fn input(&mut self, _delta: f32) {
		if self.can_continue && self.focus {
			if input::pressed(input::Control::A) && !self.current_message().no_pause {
				if self.current_phrase() + 1 == self.text.len() as u8 {
					self.finish_click = true;
				} else {
					self.current_phrase += 1;
					self.reset_phrase();
				}
			}
		}
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
		self.current_phrase() + 1 == self.text.len() as u8 &&
		if self.current_message().no_pause {
			self.timer.is_finished()
		} else {
			self.finish_click
		}
    }
}

impl Entity for DynamicText {

	fn spawn(&mut self) {
		self.focus = true;
		self.alive = true;	
		self.reset();
	}
	
	fn despawn(&mut self) {
		self.focus = false;
		self.alive = false;		
        self.timer.despawn();
		self.reset();
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}

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
			text: MessageSet::default(),
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

const fn dx() -> f32 {
	6.0
}

const fn dy() -> f32 {
	116.0
}

const fn px() -> f32 {
	11.0
}

const fn py() -> f32 {
	5.0
}