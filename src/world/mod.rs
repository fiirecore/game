use ahash::AHashMap as HashMap;
use crate::util::graphics::texture::still_texture_manager::StillTextureManager;
use crate::util::graphics::Texture;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
use crate::world::warp::WarpEntry;
use self::player::Player;

pub mod map;
pub mod warp;
pub mod npc;
pub mod pokemon;
pub mod gui;
pub mod player;
pub mod script;

mod render_coords;

pub use render_coords::RenderCoords;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = u16;

pub type TileTextures = HashMap<u16, Texture>;
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

pub struct BoundingBox(isize, isize, isize, isize);