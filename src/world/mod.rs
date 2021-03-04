use ahash::AHashMap as HashMap;
use crate::util::Coordinate;
use crate::util::graphics::texture::still_texture_manager::StillTextureManager;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
use crate::world::warp::WarpEntry;
use self::player::Player;
use self::tile::TileTextureManager;

pub mod map;
pub mod warp;
pub mod npc;
pub mod pokemon;
pub mod gui;
pub mod player;
// pub mod script;
pub mod tile;

mod render_coords;

pub use render_coords::RenderCoords;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = u16;

pub type TileTextures = TileTextureManager;
pub type NpcTextures = HashMap<String, ThreeWayTexture<StillTextureManager>>;

pub trait World {

    fn in_bounds(&self, x: isize, y: isize) -> bool;

    fn tile(&self, x: isize, y: isize) -> TileId;

    fn walkable(&self, x: isize, y: isize) -> MovementId;

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry>;

    fn on_tile(&mut self, player: &mut Player);

    fn update(&mut self, delta: f32, player: &mut Player);

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut Player);

}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct BoundingBox {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl BoundingBox {

    pub fn in_bounds(&self, coordinate: &Coordinate) -> bool{
        if coordinate.x >= self.min.x && coordinate.x <= self.max.x {
            return coordinate.y >= self.min.y && coordinate.y <= self.max.y;
        } else {
            return false;
        }
    }

    // pub fn intersects(&self, ) {

    // }

}