use parking_lot::Mutex;
use crate::io::data::text::MessageSet;

pub mod background;
pub mod text;
pub mod button;
pub mod dynamic_text;

pub mod battle;

pub mod game {
	pub mod pokemon_party_gui;
}

lazy_static::lazy_static! {
	pub static ref MESSAGE: Mutex<Option<MessageSet>> = Mutex::new(None);
}

pub fn set_message(message_set: MessageSet) {
	*MESSAGE.lock() = Some(message_set);
}

pub trait GuiComponent {

	fn load(&mut self) {}

	fn enable(&mut self);

	fn disable(&mut self);

	fn is_active(&self) -> bool;

	fn update(&mut self, _delta: f32) {}

	fn render(&self);

	fn update_position(&mut self, x: f32, y: f32);
	
}

pub trait GuiText: GuiComponent {
	
	fn get_line(&self, index: usize) -> &String;

	fn get_text(&self) -> &Vec<String>;
	
	fn get_font_id(&self) -> usize;

}

#[deprecated]
pub trait Activatable {

	fn focus(&mut self);

	fn unfocus(&mut self);

	fn in_focus(&mut self) -> bool;

	fn input(&mut self, delta: f32);

	fn next(&self) -> u8;

}

pub trait WindowManager: GuiComponent + Activatable {}