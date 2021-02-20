


use super::{GuiComponent, GuiText};
pub struct BasicButton {
	
    alive: bool,

	x: f32,
	y: f32,
	panel_x: f32,
    panel_y: f32,

    name: Vec<String>,
    font_id: usize,
	
}

impl BasicButton {
	
	pub fn new(name: &str, font_id: usize, x: f32, y: f32, panel_x: f32, panel_y: f32) -> Self {
        
		Self {

            alive: false,

			x: x,
			y: y,
			panel_x: panel_x,
            panel_y: panel_y,

            name: vec![String::from(name)],
            font_id: font_id,      
            
		}
		
	}
	
}

impl GuiComponent for BasicButton {

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel_x = x;
		self.panel_y = y;
	}
    
    fn render(&self) {
        crate::util::graphics::draw_text_left(self.get_font_id(), self.get_line(0), self.panel_x + self.x, self.panel_y + self.y);
    }

}

impl GuiText for BasicButton {

	fn get_line(&self, index: usize) -> &String {
		&self.get_text()[index]
	}

    fn get_text(&self) -> &Vec<String> {
        &self.name
    }

    fn get_font_id(&self) -> usize {
        self.font_id
    }

}

impl crate::entity::Entity for BasicButton {

	fn spawn(&mut self) {
		self.alive = true;		
	}
	
	fn despawn(&mut self) {
		self.alive = false;
	}
	
	fn is_alive(& self) -> bool {
		self.alive
    }

}