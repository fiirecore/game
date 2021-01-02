use std::collections::HashMap;

use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;

use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::{
    engine::{game_context::GameContext, text::TextRenderer},
    entity::entities::player::Player,
};

pub trait MapManager {

    fn update(&mut self, _context: &mut GameContext, _player: &Player) {

    }

    fn render_below(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        tr: &mut TextRenderer,
        textures: &HashMap<u16, Texture>,
        npc_textures: &HashMap<u8, ThreeWayTexture>,
        player: &Player,
    );

    fn render_above(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        tr: &mut TextRenderer,
        textures: &HashMap<u16, Texture>,
        player: &Player,
    );

    fn input(&mut self, context: &mut GameContext, player: &Player);

    fn on_tile(&mut self, context: &mut GameContext, player: &Player);

}
