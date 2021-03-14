use ahash::AHashMap as HashMap;
use firecore_util::Direction;
use firecore_world::TileId;
use crate::util::graphics::Texture;
use self::gui::map_window_manager::MapWindowManager;
use self::player::Player;
use self::tile::TileTextureManager;

pub mod map;
// pub mod warp;
// pub mod npc;
// pub mod pokemon;
pub mod gui;
pub mod player;
// pub mod script;
pub mod tile;

mod render_coords;

pub use render_coords::RenderCoords;

// pub type TileId = u16;
// pub type MovementId = u8;
// pub type MapSize = u16;

pub type TileTextures = TileTextureManager;
pub type NpcTextures = HashMap<String, Texture>;

pub trait GameWorld {

    fn on_tile(&mut self, player: &mut Player);

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager);

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut Player);

}

pub trait WorldNpc {

    fn interact(&mut self, direction: Option<firecore_util::Direction>, player: &mut Player);

    fn after_interact(&mut self, map_name: &String);

    fn render(&self, npc_textures: &NpcTextures, screen: &RenderCoords);

    fn current_texture_pos(&self) -> f32;

    

}

impl WorldNpc for firecore_world::character::npc::NPC {

    fn interact(&mut self, direction: Option<firecore_util::Direction>, player: &mut Player) {
        if let Some(direction) = direction {
            self.position.direction = direction.inverse();
        }
        if self.trainer.is_some() {
            self.walk_next_to(&player.position.local.coords);
            player.freeze();
        }
    }

    fn after_interact(&mut self, map_name: &String) {
        if let Some(player_data) = crate::io::data::player::PLAYER_DATA.write().as_mut() {
            // macroquad::prelude::info!("Finished interacting with {}", self.identifier.name);
            player_data.world_status.get_or_create_map_data(map_name).battle(&self);
        } else {
            macroquad::prelude::warn!("Could not get player data!");
        }
    }

    fn render(&self, npc_textures: &NpcTextures, screen: &RenderCoords) {
        let x = ((self.position.coords.x + screen.x_tile_offset) << 4) as f32 - screen.focus.x + self.position.offset.x;
        let y = ((self.position.coords.y - 1 + screen.y_tile_offset) << 4) as f32 - screen.focus.y + self.position.offset.y;
        
        if let Some(texture) = npc_textures.get(&self.identifier.npc_type) {
            macroquad::prelude::draw_texture_ex(*texture, x, y, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
                source: Some(macroquad::prelude::Rect::new(
                    self.current_texture_pos(),
                    0.0,
                    16.0,
                    32.0,
                )),
                flip_x: self.position.direction == Direction::Right,
                ..Default::default()
            })
        } else {
            crate::util::graphics::draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16, 16);
        }
    }

    fn current_texture_pos(&self) -> f32 {
        match self.position.direction {
            Direction::Down => 0.0,
            Direction::Up => 16.0,
            _ => 32.0
        }
        // (
		// 	*self.texture_index()
		// 		.get(
		// 			(
		// 				if self.position.offset.x != 0.0 {
		// 					self.position.offset.x
		// 				} else {
		// 					self.position.offset.y
		// 				}.abs() as usize >> 3
		// 			) + self.sprite_index as usize
		// 		).unwrap_or(
		// 			&3
		// 		)
		// 	<< 4
		// ) as f32
    }

}