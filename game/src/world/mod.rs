// extern crate firecore_game as game;

use crate::battle_glue::BattleEntryRef;

use deps::tetra::Context;
use worldlib::{
    character::player::PlayerCharacter,
    map::{
        World,
        warp::WarpDestination,
        manager::{TrainerEntryRef, Door},
    },
};

use map::texture::WorldTextures;
use self::gui::TextWindow;

pub mod map;
pub mod npc;
pub mod gui;
pub mod battle;

mod render_coords;

pub use render_coords::RenderCoords;

pub trait GameWorld: World {

    fn on_start(&mut self, ctx: &mut Context, music: bool);

    fn on_tile(&mut self, ctx: &mut Context, battle: BattleEntryRef/*, trainer: TrainerEntryRef*/, player: &mut PlayerCharacter);

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, trainer: TrainerEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow);

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, door: &Option<Door>, screen: &RenderCoords, border: bool);

}

pub fn seed_randoms(seed: u64) {
    firecore_world_lib::map::wild::WILD_RANDOM.seed(seed);
	map::NPC_RANDOM.seed(seed);
}