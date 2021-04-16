extern crate firecore_game as game;
extern crate firecore_world_lib as world;

use game::{
    util::{
        hash::HashMap,
        tinystr::TinyStr16,
    },
    battle::BattleData,
    macroquad::prelude::Texture2D,
};

use world::{
    character::player::PlayerCharacter,
    map::warp::WarpDestination,
};

use self::gui::text_window::TextWindow;
use self::tile::TileTextureManager;

pub mod map;
pub mod npc;
pub mod gui;
pub mod player;
pub mod tile;
pub mod battle;
pub mod warp_transition;

mod render_coords;

pub use render_coords::RenderCoords;

pub type TileTextures = TileTextureManager;
pub type NpcTextures = HashMap<TinyStr16, Texture2D>;

pub trait GameWorld {

    fn on_start(&mut self, music: bool);

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter);

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow);

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter);

}

pub fn seed_randoms(seed: u64) {
    firecore_world_lib::map::wild::WILD_RANDOM.seed(seed);
	crate::map::NPC_RANDOM.seed(seed);
}