use ahash::AHashMap as HashMap;
use firecore_world::TileId;
use firecore_world::character::npc::npc_type::NPCType;
use macroquad::prelude::Texture2D;
use crate::battle::data::BattleData;

use self::gui::text_window::TextWindow;
use firecore_world::character::player::PlayerCharacter;
use self::tile::TileTextureManager;

pub mod map;
pub mod npc;
pub mod gui;
pub mod player;
pub mod tile;

pub mod warp_transition;

mod render_coords;

pub use render_coords::RenderCoords;

pub type NPCTypes = HashMap<String, NPCType>;
pub type TileTextures = TileTextureManager;
pub type NpcTextures = HashMap<String, Texture2D>;
pub type GuiTextures = HashMap<u8, Texture2D>;

pub trait GameWorld {

    fn on_start(&self, music: bool);

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter);

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, text_window: &mut TextWindow, npc_types: &NPCTypes);

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter);

}