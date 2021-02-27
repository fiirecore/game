use crate::util::GlobalPosition;
use crate::util::Render;
use crate::util::graphics::texture::movement_texture::MovementTexture;
use crate::util::graphics::texture::movement_texture_manager::MovementTextureManager;
use crate::util::graphics::texture::texture_manager::TextureManager;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
use crate::util::Direction;
use crate::io::data::player::PlayerData;
use crate::util::TILE_SIZE;

use crate::util::graphics::draw_flip;
use crate::util::graphics::texture::byte_texture;

static TEX_TICK_LENGTH: f32 = 8.0 / 60.0;
pub static BASE_SPEED: u8 = 1;
pub static RUN_SPEED: u8 = BASE_SPEED << 1;

#[derive(Default)]
pub struct Player {
	
	pub position: GlobalPosition,

	pub speed: u8,

	textures: Vec<ThreeWayTexture<MovementTextureManager>>,
	
	pub moving: bool,
	pub running: bool,
	pub frozen: bool,

	pub noclip: bool,
	
}

impl Player {

	pub fn new(data: &PlayerData) -> Player {
		
		Player {
			
			position: data.location.position,
			speed: BASE_SPEED,

			..Default::default()
			
		}
		
	}

	pub fn move_update(&mut self, delta: f32) {
		self.textures[0].update_with_direction(delta, self.position.local.direction);
		self.textures[1].update_with_direction(delta, self.position.local.direction);
	}

	pub fn on_try_move(&mut self, direction: Direction) {
		self.position.local.direction = direction;
		self.textures[0].direction = direction.value();
		self.textures[0].unidle();
	}

	pub fn on_stopped_moving(&mut self) {
		self.textures[0].idle();
	}

	pub fn freeze(&mut self) {
		self.frozen = true;
		self.position.local.x_offset = 0.0;
		self.position.local.y_offset = 0.0;
		self.moving = false;
		self.running = false;
		self.speed = BASE_SPEED;
		self.on_stopped_moving();
	}
	
}

static SCREEN_X: f32 = ((crate::BASE_WIDTH as isize - TILE_SIZE as isize) >> 1) as f32 + 0.0;
static SCREEN_Y: f32 = ((crate::BASE_HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - 4.0;

impl Render for Player {
	
	fn render(&self) {
		//let tex = self.move_textures[self.move_status as usize].get_texture();
		let tex;
		if self.running && self.moving {
			tex = self.textures[1].texture();
		} else {
			tex = self.textures[0].texture();
		}
		draw_flip(tex.0, SCREEN_X, SCREEN_Y, tex.1);
	}

}

impl Player {

	pub fn load_textures(&mut self) { // To - do: Use NPC styled spritesheet for player that is found in the code

		let mut down_textures = MovementTexture::empty((0, false));

		down_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/idle_down.png"))); 
		down_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/walk_down_l.png")));
		//down_textures.push_texture(byte_texture(path.join("walk_down_r.png"));

		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, false);
		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, true);


		let mut up_textures = MovementTexture::empty((0, false));

		up_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/idle_up.png"))); 
		up_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/walk_up_l.png")));
		//up_textures.push_texture(byte_texture(path.join("walk_up_r.png"));

		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, false);
		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, true);

		let mut side_textures = MovementTexture::empty((0, false));

		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/idle_side.png")));
		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/walk_side_l.png")));
		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/walk_side_r.png")));

		side_textures.map_to_index(0, false);
		side_textures.map_to_index(1, false);
		side_textures.map_to_index(0, false);
		side_textures.map_to_index(2, false);

		let mut walk_textures = ThreeWayTexture::new();

		walk_textures.add_texture_manager(MovementTextureManager::new(down_textures, TEX_TICK_LENGTH));
		walk_textures.add_texture_manager(MovementTextureManager::new(up_textures, TEX_TICK_LENGTH));
		walk_textures.add_texture_manager(MovementTextureManager::new(side_textures, TEX_TICK_LENGTH));

		self.textures.push(walk_textures);



		let mut up_textures = MovementTexture::empty((0, false));

		up_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_up.png"))); 
		up_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_up_l.png")));
		//up_textures.push_texture(byte_texture(path.join("run_up_r.png"));

		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, false);
		up_textures.map_to_index(0, false);
		up_textures.map_to_index(1, true);

		
		let mut down_textures = MovementTexture::empty((0, false));

		down_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_down.png"))); 
		down_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_down_l.png")));
		//down_textures.push_texture(byte_texture(path.join("run_down_r.png"));

		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, false);
		down_textures.map_to_index(0, false);
		down_textures.map_to_index(1, true);

		let mut side_textures = MovementTexture::empty((0, false));

		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_side.png")));
		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_side_l.png")));
		side_textures.push_texture(byte_texture(include_bytes!("../../build/assets/textures/player/run_side_r.png")));

		side_textures.map_to_index(0, false);
		side_textures.map_to_index(1, false);
		side_textures.map_to_index(0, false);
		side_textures.map_to_index(2, false);

		let mut run_textures = ThreeWayTexture::new();

		run_textures.add_texture_manager(MovementTextureManager::new(down_textures, TEX_TICK_LENGTH / 2.0));
		run_textures.add_texture_manager(MovementTextureManager::new(up_textures, TEX_TICK_LENGTH / 2.0));
		run_textures.add_texture_manager(MovementTextureManager::new(side_textures, TEX_TICK_LENGTH / 2.0));

		self.textures.push(run_textures);


	}

}