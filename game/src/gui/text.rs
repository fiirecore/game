use firecore_text::{
	FontId,
	message::{Message, MessagePages, MessagePage, TextColor},
};
use util::{Entity, Timer, Reset, Completable};
use input::{pressed, Control};

use macroquad::prelude::Vec2;

use crate::graphics::{draw_text_left, draw_button};

#[derive(Clone)]
pub struct DynamicText {

	alive: bool,
    
	origin: Vec2,
	
	pub font: FontId,
	pub message: Message,
	pub current: usize,
	current_line: usize,

	counter: f32,
	
	pub can_continue: bool,
	finish_click: bool,
	timer: Timer,

	button: (f32, bool),

}

impl DynamicText {

	pub fn new(origin: Vec2, font: FontId, color: TextColor, len: usize) -> Self {
		Self {
			alive: false,

			origin,

			font,
			message: Message::empty(color, len),
			current: 0,
			current_line: 0,

			counter: 0.0,

			can_continue: false,
			finish_click: false,
			timer: Timer::new(false, 1.0),
			
			button: Default::default(),
		}
	}


	pub fn set(&mut self, pages: MessagePages) {
		self.message.pages = pages;
	}

	pub fn push(&mut self, page: MessagePage) {
		self.message.pages.push(page);
	}

	pub fn remove(&mut self, index: usize) {
		self.message.pages.remove(index);
	}

	pub fn clear(&mut self) {
		self.message.pages.clear();
	}

	pub fn len(&self) -> usize {
		self.message.pages.len()
	}

	pub fn current(&self) -> usize {
		self.current
	}

	pub fn can_continue(&self) -> bool {
		self.can_continue
	}

	pub fn process_messages(&mut self, save: &pokemon_firered_clone_storage::player::PlayerSave) {
		crate::text::process_messages(&mut self.message.pages, save);
	}

	fn reset_page(&mut self) {
		self.can_continue = false;
		self.current_line = 0;
		self.counter = 0.0;
		self.timer.hard_reset();
	}

	pub fn update(&mut self, delta: f32) {
		if self.alive {
			if let Some(current) = self.message.pages.get(self.current) {
				let line_len = current.lines[self.current_line].len() << 2;
				if self.can_continue {

					if pressed(Control::A) && current.wait.is_none() {
						if self.current + 1 >= self.len() {
							self.finish_click = true;
						} else {
							self.current += 1;
							self.reset_page();
						}
						return;
					}
					
					if let Some(time) = current.wait {
						if !self.timer.is_alive() {
							self.timer.spawn();
							self.timer.length = time;
						}
						self.timer.update(delta);
						if self.timer.is_finished() {
							if self.current + 1 != self.len() {
								self.current += 1;
								self.reset_page();
								self.timer.soft_reset();
								self.timer.despawn();
							}
						}
					}
					if self.button.1 {
						self.button.0 += delta * 7.5;
						if self.button.0 > 3.0 {
							self.button.1 = false;
						}
					} else {
						self.button.0 -= delta * 7.5;
						if self.button.0 < -3.0 {
							self.button.1 = true;
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
			if let Some(current_line) = self.message.pages.get(self.current).map(|page| page.lines.get(self.current_line)).flatten() {
				let string = if current_line.len() > (self.counter as usize) >> 2 {
					&current_line[..(self.counter as usize) >> 2]
				} else {
					current_line
				};

				let current = &self.message.pages[self.current];

				let y = (self.current_line << 4) as f32;
				draw_text_left(self.font, string, self.message.color, self.origin.x, self.origin.y + y);

				for index in 0..self.current_line {
					draw_text_left(self.font, &current.lines[index], self.message.color, self.origin.x, self.origin.y + (index << 4) as f32);
				}

				if self.can_continue && current.wait.is_none() {
					draw_button(current_line, self.font, self.origin.x, self.origin.y + 2.0 + self.button.0 + y);
				}
			}
		}
	}

}

impl Reset for DynamicText {
    fn reset(&mut self) {
        self.current = 0;
		self.button = Default::default();
		self.finish_click = false;
		self.reset_page();
    }
}

impl Completable for DynamicText {
    fn is_finished(&self) -> bool {
		self.current + 1 == self.len() && if self.message.pages.get(self.current).map(|page| page.wait.is_none()).unwrap_or_default() {
			self.finish_click
		} else {
			self.timer.is_finished()
		} &&
		self.can_continue
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
		self.clear();
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}

}