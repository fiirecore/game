// extern crate firecore_game as game;

use crate::game::battle_glue::BattleEntryRef;

use engine::tetra::{graphics::Color, Context};

use worldlib::map::{manager::WorldMapManagerData, World};

use self::{gui::TextWindow, map::manager::Door, map::texture::WorldTextures};

pub mod battle;
pub mod gui;
pub mod map;
pub mod npc;

mod screen;

pub use screen::RenderCoords;

pub trait GameWorld: World {
    fn on_start(&mut self, ctx: &mut Context, music: bool);

    fn on_tile(&mut self, world: &mut WorldMapManagerData, battle: BattleEntryRef);

    fn update(
        &mut self,
        ctx: &mut Context,
        delta: f32,
        world: &mut WorldMapManagerData,
        battle: BattleEntryRef,
        text_window: &mut TextWindow,
    );

    fn draw(
        &self,
        ctx: &mut Context,
        textures: &WorldTextures,
        door: &Option<Door>,
        screen: &RenderCoords,
        border: bool,
        color: Color,
    );
}
