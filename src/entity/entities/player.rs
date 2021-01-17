use std::path::PathBuf;

use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use crate::util::context::GameContext;
use crate::entity::texture::movement_texture::MovementTexture;
use crate::entity::texture::movement_texture_manager::MovementTextureManager;
use crate::entity::texture::texture_manager::TextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::io::data::Direction;
use crate::io::data::Position;

use crate::io::data::player_data::PlayerData;

use crate::util::file::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;
use crate::util::render_util::draw_flip;
use crate::entity::*;
use crate::util::render_util::TEXTURE_SIZE;
use crate::util::traits::Loadable;

static TEX_TICK_LENGTH: u8 = 8;
pub static RUN_SPEED: u8 = 2;

pub struct Player {
	
	alive: bool,
	
	pub position: Position,
	
	pub moving: bool,
	//pub dir_changed: bool,
	
	//world_id: String,
	//move_status: u8,
	//move_textures: Vec<MobMoveTexture>,
	textures: Vec<ThreeWayTexture>,
	
	pub running: bool,

	pub speed: u8,
	
	pub screen_attached: bool,
	pub focus_x: isize,
	pub focus_y: isize,

	pub noclip: bool,

	pub frozen: bool,
	
}

impl Default for Player {

    fn default() -> Self {

        Self {

			alive: false,

			position: Position::default(),

			moving: false,
			//dir_changed: false,

			textures: Vec::new(),
		
			running: false,
		
			speed: 0,
		
			screen_attached: false,
			focus_x: 0,
			focus_y: 0,

			noclip: false,

			frozen: false,

		}

    }
}

impl Player {

	
	
	pub fn new(data: &PlayerData) -> Player {
		
		Player {
			
			alive: true,

			position: data.location.position,
			
			speed: 1,
			
			screen_attached: true,

			..Default::default()
			
		}
		
	}

	pub fn save_data(&self, data: &mut PlayerData) {
		data.location.position = self.position;
	}
	
	pub fn focus_update(&mut self) {
		if self.screen_attached {
			self.focus_x = self.position.pixel_x() + TEXTURE_SIZE as isize / 2 - crate::BASE_WIDTH as isize / 2;
			self.focus_y = self.position.pixel_y() + TEXTURE_SIZE as isize / 2 - crate::BASE_HEIGHT as isize / 2;
		}
	}

	pub fn move_update(&mut self) {
		self.textures[0].update_with_direction(self.position.direction.value());
		self.textures[1].update_with_direction(self.position.direction.value());
	}

	pub fn reset_speed(&mut self) {
		self.running = false;
		self.speed = 1;
	}

	pub fn moving(&mut self) {
		self.moving = true;
	}

	pub fn on_try_move(&mut self, direction: Direction) {
		self.textures[0].direction = direction.value();
		self.textures[0].unidle();
	}

	pub fn on_stopped_moving(&mut self) {
		self.textures[0].idle();
	}
	
}

impl Loadable for Player {

	fn load(&mut self) {

	}

}

impl Entity for Player {
	
	fn spawn(&mut self) {
		self.alive = true;
	}
	
	fn despawn(&mut self) {
		self.alive = false;
	}
	
	fn is_alive(&self) -> bool {
		self.alive
	}
	
}

impl Ticking for Player {

	fn update(&mut self, _context: &mut GameContext) {
		if self.is_alive() {

		}
	}
	
	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		if self.alive {
			//let tex = self.move_textures[self.move_status as usize].get_texture();
			let tex;
			if self.running {
				tex = self.textures[1].texture();
			} else {
				tex = self.textures[0].texture();
			}
			draw_flip(ctx, g, tex.0, self.position.pixel_x() - self.focus_x, self.position.pixel_y() - self.focus_y - 4 /* 20 - 4 = 16, on tile border */, tex.1);
		}
		
	}

}

/*

impl PositionedEntity for Player {
	
	fn get_px(&mut self) -> isize {
		self.position.pixel_x()
	}
	
	fn get_py(&mut self) -> isize {
		self.position.pixel_y()
	}
	
	fn move_entity(&mut self, _direction: Direction) {
		
	}
	
}

impl Mob for Player {
	
	fn get_speed(&mut self) -> u8 {
		self.speed
	} 
	
}

*/

impl Player {

	pub fn load_textures(&mut self) {

		let mut path = asset_as_pathbuf("world");
		//path.push(world_id); // fix
		path.push("textures/player");
		// if !path.exists() {
		// 	path.pop();
		// 	path.pop();
		// 	path.push("textures/player");
		// };

		let mut up_textures = MovementTexture::empty((0, false));

		up_textures.push_texture(Player::get_texture(&path, "idle_up.png")); 
		up_textures.push_texture(Player::get_texture(&path, "walk_up_l.png"));
		//up_textures.push_texture(Player::get_texture(&path, "walk_up_r.png"));

		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, false);
		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, true);

		
		let mut down_textures = MovementTexture::empty((0, false));

		down_textures.push_texture(Player::get_texture(&path, "idle_down.png")); 
		down_textures.push_texture(Player::get_texture(&path, "walk_down_l.png"));
		//down_textures.push_texture(Player::get_texture(&path, "walk_down_r.png"));

		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, false);
		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, true);

		let mut side_textures = MovementTexture::empty((0, false));

		side_textures.push_texture(Player::get_texture(&path, "idle_side.png"));
		side_textures.push_texture(Player::get_texture(&path, "walk_side_l.png"));
		side_textures.push_texture(Player::get_texture(&path, "walk_side_r.png"));

		side_textures.map_to_index(0, false);
		side_textures.map_to_index(1, false);
		side_textures.map_to_index(0, false);
		side_textures.map_to_index(2, false);

		let mut walk_textures = ThreeWayTexture::new();

		walk_textures.add_texture_manager(Box::new(MovementTextureManager::new(up_textures, TEX_TICK_LENGTH as usize)));
		walk_textures.add_texture_manager(Box::new(MovementTextureManager::new(down_textures, TEX_TICK_LENGTH as usize)));
		walk_textures.add_texture_manager(Box::new(MovementTextureManager::new(side_textures, TEX_TICK_LENGTH as usize)));

		self.textures.push(walk_textures);



		let mut up_textures = MovementTexture::empty((0, false));

		up_textures.push_texture(Player::get_texture(&path, "run_up.png")); 
		up_textures.push_texture(Player::get_texture(&path, "run_up_l.png"));
		//up_textures.push_texture(Player::get_texture(&path, "run_up_r.png"));

		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, false);
		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, true);

		
		let mut down_textures = MovementTexture::empty((0, false));

		down_textures.push_texture(Player::get_texture(&path, "run_down.png")); 
		down_textures.push_texture(Player::get_texture(&path, "run_down_l.png"));
		//down_textures.push_texture(Player::get_texture(&path, "run_down_r.png"));

		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, false);
		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, true);

		let mut side_textures = MovementTexture::empty((0, false));

		side_textures.push_texture(Player::get_texture(&path, "run_side.png"));
		side_textures.push_texture(Player::get_texture(&path, "run_side_l.png"));
		side_textures.push_texture(Player::get_texture(&path, "run_side_r.png"));

		side_textures.map_to_index(0, false);
		side_textures.map_to_index(1, false);
		side_textures.map_to_index(0, false);
		side_textures.map_to_index(2, false);

		let mut run_textures = ThreeWayTexture::new();

		run_textures.add_texture_manager(Box::new(MovementTextureManager::new(up_textures, TEX_TICK_LENGTH as usize / 2)));
		run_textures.add_texture_manager(Box::new(MovementTextureManager::new(down_textures, TEX_TICK_LENGTH as usize / 2)));
		run_textures.add_texture_manager(Box::new(MovementTextureManager::new(side_textures, TEX_TICK_LENGTH as usize / 2)));

		self.textures.push(run_textures);


	}

	fn get_texture(path: &PathBuf, file: &str) -> Texture {
		let mut path = path.clone();
		path.push(file);
		return texture_from_path(path);
	}

}