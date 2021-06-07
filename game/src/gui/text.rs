use firecore_text::{
	FontId,
	message::{Message, MessagePages, MessagePage, TextColor},
};
use util::{Entity, Timer, Reset, Completable};
use crate::input::{pressed, Control};

use crate::tetra::{Context, math::Vec2};

use crate::graphics::{draw_text_left, draw_button};

#[derive(Clone)]
pub struct TextDisplay {

	alive: bool,
    
	origin: Vec2<f32>,
	
	pub font: FontId,
	pub message: Message,
	pub current: usize,
	current_line: usize,

	counter: f32,
	
	pub can_continue: bool,
	end: bool,
	timer: Timer,

	button: (f32, bool),

}

impl TextDisplay {

	pub fn new(origin: Vec2<f32>, font: FontId, color: TextColor, len: usize) -> Self {
		Self {
			alive: false,

			origin,

			font,
			message: Message::empty(color, len),
			current: 0,
			current_line: 0,

			counter: 0.0,

			can_continue: false,
			end: false,
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

	pub fn color(&mut self, color: TextColor) {
		self.message.color = color;
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

	pub fn update(&mut self, ctx: &Context, delta: f32) {
		if self.alive {
			if let Some(current) = self.message.pages.get(self.current) {
				let line_len = current.lines[self.current_line].len() << 2;
				if self.can_continue {
					match current.wait {
						Some(wait) => {
							if !self.timer.alive() {
								self.timer.hard_reset();
								self.timer.spawn();
								self.timer.length = wait;
							} else {
								self.timer.update(delta);
								if self.timer.finished() {
									self.timer.despawn();
									if self.current + 1 >= self.len() {
										self.end = true;
									} else {
										self.current += 1;
										self.reset_page();
									}
								}
							}
						}
						None => {
						
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

							if pressed(ctx, Control::A) {
								if self.current + 1 >= self.len() {
									self.end = true;
								} else {
									self.current += 1;
									self.reset_page();
								}
							}
						}
					}

				} else if self.counter <= line_len as f32 {
					self.counter += delta * 120.0;
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

	pub fn draw(&self, ctx: &mut Context) {
		if self.alive {
			if let Some(current_line) = self.message.pages.get(self.current).map(|page| page.lines.get(self.current_line)).flatten() {
				let string = if current_line.len() > (self.counter as usize) >> 2 {
					&current_line[..(self.counter as usize) >> 2]
				} else {
					current_line
				};

				let current = &self.message.pages[self.current];

				let y = (self.current_line << 4) as f32;
				draw_text_left(ctx, &self.font, string, &self.message.color, self.origin.x, self.origin.y + y);

				for index in 0..self.current_line {
					draw_text_left(ctx, &self.font, &current.lines[index], &self.message.color, self.origin.x, self.origin.y + (index << 4) as f32);
				}

				if self.can_continue && current.wait.is_none() {
					draw_button(ctx, &self.font, current_line, self.origin.x, self.origin.y + 2.0 + self.button.0 + y);
				}
			}
		}
	}

}

impl Reset for TextDisplay {
    fn reset(&mut self) {
        self.current = 0;
		self.button = Default::default();
		self.end = false;
		self.reset_page();
    }
}

impl Completable for TextDisplay {
    fn finished(&self) -> bool {
		(self.current + 1 >= self.len() && 
		self.end &&
		self.can_continue) ||
		self.len() == 0
    }
}

impl Entity for TextDisplay {

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
	
	fn alive(& self) -> bool {
		self.alive
	}

}