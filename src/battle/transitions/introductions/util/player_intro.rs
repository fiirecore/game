use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw;
use crate::util::texture_util::texture_from_path;
use crate::util::traits::Loadable;

pub struct PlayerBattleIntro {

	player_textures: Vec<Texture>,
	player_x_counter: u8,
	player_texture_index: u8,

}

impl PlayerBattleIntro {

    pub fn new() -> Self {

        Self {

			player_textures: Vec::new(),
			player_x_counter: 0,
			player_texture_index: 0,

        }

    }

    pub fn should_update(&self) -> bool {
        return self.player_x_counter < 41 + 63;
    }

    pub fn update(&mut self) {
        if self.player_texture_index == 0 {
            self.player_texture_index = 1;
        } else if self.player_texture_index == 1 && self.player_x_counter == 42 {
            self.player_texture_index = 2;
        } else if self.player_texture_index == 2 && self.player_x_counter == 60 {
            self.player_texture_index = 3;
        } else if self.player_texture_index == 3 && self.player_x_counter == 78 {
            self.player_texture_index = 4;
        } else {
            self.player_x_counter+=3;
        }
    }

    pub fn draw(&self, ctx: &mut Context, g: &mut GlGraphics, offset: u16) {
        draw(ctx, g, &self.player_textures[self.player_texture_index as usize], 41 + offset as isize - self.player_x_counter as isize, 64);
    }

    pub fn reset(&mut self) {
        self.player_x_counter = 0;
		self.player_texture_index = 0;
    }

}

impl Loadable for PlayerBattleIntro {

    fn load(&mut self) {
        self.player_textures.push(texture_from_path(asset_as_pathbuf("gui/battle/player0.png")));
        self.player_textures.push(texture_from_path(asset_as_pathbuf("gui/battle/player1.png")));
        self.player_textures.push(texture_from_path(asset_as_pathbuf("gui/battle/player2.png")));
        self.player_textures.push(texture_from_path(asset_as_pathbuf("gui/battle/player3.png")));
        self.player_textures.push(texture_from_path(asset_as_pathbuf("gui/battle/player4.png")));
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        self.reset();
    }

}