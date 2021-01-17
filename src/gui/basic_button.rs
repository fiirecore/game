use piston_window::Context;
use opengl_graphics::GlGraphics;
use crate::util::text_renderer::TextRenderer;
use crate::gui::gui::{GuiComponent, GuiText};
pub struct BasicButton {
	
    alive: bool,

	x: isize,
	y: isize,
	panel_x: isize,
    panel_y: isize,

    name: Vec<String>,
    font_id: usize,
	
}

impl BasicButton {
	
	pub fn new(name: &str, font_id: usize, x: isize, y: isize, panel_x: isize, panel_y: isize) -> Self {
        
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
	
	fn enable(&mut self) {
		self.alive = true;		
	}
	
	fn disable(&mut self) {
		self.alive = false;
	}
	
	fn is_active(& self) -> bool {
		self.alive
    }

	fn update_position(&mut self, x: isize, y: isize) {
		self.panel_x = x;
		self.panel_y = y;
	}
    
    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        tr.render_text_from_left(ctx, g, self.get_font_id(), self.get_line(0), self.panel_x + self.x, self.panel_y + self.y);
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