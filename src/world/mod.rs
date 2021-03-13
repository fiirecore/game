use ahash::AHashMap as HashMap;
use firecore_world::TileId;
use crate::util::graphics::texture::still_texture_manager::StillTextureManager;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
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
pub type NpcTextures = HashMap<String, ThreeWayTexture<StillTextureManager>>;

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

}

impl WorldNpc for firecore_world::npc::NPC {

    fn interact(&mut self, direction: Option<firecore_util::Direction>, player: &mut Player) {
        if let Some(direction) = direction {
            self.position.direction = direction.inverse();
        }
        if self.trainer.is_some() {
            macroquad::prelude::info!("Trainer battle with {}", &self.identifier.name);
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
        if let Some(twt) = npc_textures.get(&self.identifier.npc_type) {
            let tuple = twt.of_direction(self.position.direction);
            crate::util::graphics::draw_flip(tuple.0, x, y, tuple.1);
        } else {
            crate::util::graphics::draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16, 16);
        }
    }

}