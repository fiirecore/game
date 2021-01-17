use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::util::file::asset_as_pathbuf;
use crate::util::render_util::draw_o;
use crate::util::texture_util::texture_from_path;
use crate::util::traits::Loadable;

pub struct BattleBackground {

	background_texture: Option<Texture>,
	ground_texture: Option<Texture>,

}

impl BattleBackground {

    pub fn new() -> Self {

        Self {

            background_texture: None,
            ground_texture: None,

        }

    }

    pub fn render(&self, ctx: &mut Context, g: &mut GlGraphics, offset: u16) {
        draw_o(ctx, g, self.background_texture.as_ref(), 0, 1);
        draw_o(ctx, g, self.ground_texture.as_ref(), 113 - offset as isize, 50);
		draw_o(ctx, g, self.ground_texture.as_ref(), 0 + offset as isize, 103);
    }

}

impl Loadable for BattleBackground {

    fn load(&mut self) {
        self.background_texture = Some(texture_from_path(asset_as_pathbuf("gui/battle/background.png")));
		self.ground_texture = Some(texture_from_path(asset_as_pathbuf("gui/battle/grass_pad.png")));
    }

}