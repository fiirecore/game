use serde::Deserialize;
use crate::gui::background::Background;
use crate::util::Entity;
use super::player::Player;

pub mod npc;

#[derive(Clone, Deserialize)]
pub enum ScriptRunType {

    Once,
    Conditional, // check player data for conditions
    Always,
    AlwaysNoReset,

}

pub trait WorldScript: Entity {

    fn start(&mut self, player: &mut Player);

    fn finish(&mut self, player: &mut Player);

    fn update(&mut self, delta: f32, player: &mut super::Player);

    fn render(&self, tile_textures: &super::TileTextures, npc_textures: &super::NpcTextures, background: &Background, screen: &super::RenderCoords);

    fn has_run(&self) -> bool;

}