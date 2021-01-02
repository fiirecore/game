use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

pub trait Ticking {

	fn update(&mut self, _context: &mut GameContext) {
		
	}
	
	fn render(&self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {
		
	}

}

/*

pub trait PositionedEntity: Ticking {
	
	fn get_px(&mut self) -> isize;
	
	fn get_py(&mut self) -> isize;
	
	fn move_entity(&mut self, direction: Direction);
	
	fn is_solid(&mut self) -> bool {
		true
	}
	
}

pub trait Mob: PositionedEntity {
	
	fn get_speed(&mut self) -> u8 {
		2
	}
	
}

pub trait NPC: Mob {
	
	fn interact(&mut self);
	
	fn pathfind(&mut self);
	
}

pub trait Trainer: NPC {
	
	fn check_sight(&mut self);
	
}

*/

