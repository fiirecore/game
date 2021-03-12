use firecore_util::text::MessageSet;
use parking_lot::Mutex;

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

pub trait GuiComponent: firecore_util::Entity {

	fn on_start(&mut self) {}

	fn update(&mut self, _delta: f32) {}

	fn render(&self);

	fn update_position(&mut self, x: f32, y: f32);
	
}

pub trait GuiText: GuiComponent {
	
	fn get_line(&self, index: usize) -> &String;

	fn get_text(&self) -> &Vec<String>;
	
	fn get_font_id(&self) -> usize;

}

pub trait Focus {
	
	fn focus(&mut self);

	fn unfocus(&mut self);

	fn in_focus(&mut self) -> bool;

}

pub trait WindowManager: GuiComponent + crate::util::Input + Focus {}