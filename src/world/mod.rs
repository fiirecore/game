use ahash::AHashMap as HashMap;
use firecore_world::TileId;
use crate::util::graphics::Texture;
use self::gui::map_window_manager::MapWindowManager;
use self::player::Player;
use self::tile::TileTextureManager;

pub mod map;
pub mod npc;
pub mod gui;
pub mod player;
pub mod tile;

mod render_coords;

pub use render_coords::RenderCoords;

pub type TileTextures = TileTextureManager;
pub type NpcTextures = HashMap<String, Texture>;

pub trait GameWorld {

    fn on_tile(&mut self, player: &mut Player);

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager);

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut Player);

}