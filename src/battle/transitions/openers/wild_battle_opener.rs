use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::engine::Texture;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleOpener;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw;

use crate::util::texture_util::texture_from_path;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

use super::trainer_battle_opener::TrainerBattleOpener;
pub struct WildBattleOpener {

    //active: bool,
    //finished: bool,

    trainer_battle_opener: TrainerBattleOpener,
    
    grass: Texture,
    
    grass_active: bool,
    grass_x_offset: i16,
    grass_y_offset: u8,

}

static GRASS_X_OFFSET: i16 = 128; // width of image
static GRASS_Y_OFFSET: u8 = 47; // height of image

impl WildBattleOpener {

    pub fn new() -> Self {

        Self {

            //active: false,
            //finished: false,

            trainer_battle_opener: TrainerBattleOpener::new(),

            grass_active: true,
            grass_x_offset: GRASS_X_OFFSET,
            grass_y_offset: GRASS_Y_OFFSET,
            grass: texture_from_path(asset_as_pathbuf("gui/battle/grass.png")),
        }

    }

}

impl BattleTransition for WildBattleOpener {

    fn reset(&mut self) {
        self.grass_active = true;
        self.grass_x_offset = GRASS_X_OFFSET;
        self.grass_y_offset = GRASS_Y_OFFSET;
        self.trainer_battle_opener.reset();
    }
    
}

impl Loadable for WildBattleOpener {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.trainer_battle_opener.on_start(context);
    } 

}

impl Completable for WildBattleOpener {

    fn is_finished(&self) -> bool {
        return self.trainer_battle_opener.is_finished();
    }

}

impl Ticking for WildBattleOpener {

    fn update(&mut self, context: &mut GameContext) {
        self.trainer_battle_opener.update(context);
        if self.grass_active {
            self.grass_x_offset -= 6;
            if self.grass_x_offset < 0 {
                self.grass_x_offset += GRASS_X_OFFSET;
            }
            if self.trainer_battle_opener.offset() <= 130 && self.trainer_battle_opener.offset() % 2 == 0 {
                self.grass_y_offset -= 1;
            }
            if self.grass_y_offset <= 0 {
                self.grass_active = false;
            }
        }
        
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.trainer_battle_opener.render(ctx, g, tr);
    }

}

impl BattleOpener for WildBattleOpener {
    fn offset(&self) -> u16 {
        return self.trainer_battle_opener.offset();
    }

    fn render_below_panel(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        if self.grass_active {
            let y = 114 - self.grass_y_offset as isize;
            draw(
                ctx,
                g,
                &self.grass,
                self.grass_x_offset as isize - GRASS_X_OFFSET as isize,
                y,
            );
            draw(
                ctx,
                g,
                &self.grass,
                self.grass_x_offset as isize as isize,
                y,
            );
            draw(
                ctx,
                g,
                &self.grass,
                self.grass_x_offset as isize + GRASS_X_OFFSET as isize,
                y,
            );
        }
    }
}

impl Entity for WildBattleOpener {
    fn spawn(&mut self) {
        self.reset();
        //self.active = true;
        self.trainer_battle_opener.spawn();
    }

    fn despawn(&mut self) {
        //self.active = false;
        //self.finished = false;
        self.trainer_battle_opener.despawn();
    }

    fn is_alive(&self) -> bool {
        //self.active
        return self.trainer_battle_opener.is_alive();
    }
}
