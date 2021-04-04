// pub mod panel;
pub mod text;
pub mod game;


pub trait GuiText {
	
	fn get_line(&self, index: usize) -> &String;

	fn get_text(&self) -> &Vec<String>;
	
	fn get_font_id(&self) -> usize;

}