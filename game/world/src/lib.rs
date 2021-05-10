extern crate firecore_game as game;
extern crate firecore_world_lib as world;

use game::battle::BattleData;

use world::{
    character::player::PlayerCharacter,
    map::warp::WarpDestination,
};

use map::texture::WorldTextures;
use self::gui::text_window::TextWindow;

pub mod map;
pub mod npc;
pub mod gui;
pub mod battle;

mod render_coords;

pub use render_coords::RenderCoords;

pub trait GameWorld {

    fn on_start(&mut self, music: bool);

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, character: &mut PlayerCharacter);

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow);

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter);

}

pub fn seed_randoms(seed: u64) {
    firecore_world_lib::map::wild::WILD_RANDOM.seed(seed);
	crate::map::NPC_RANDOM.seed(seed);
}