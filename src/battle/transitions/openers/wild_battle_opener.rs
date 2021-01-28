use crate::util::texture::Texture;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleOpener;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render::draw;
use crate::util::{Reset, Completable};
use crate::util::Load;
use super::trainer_battle_opener::TrainerBattleOpener;
pub struct WildBattleOpener {

    //active: bool,
    //finished: bool,

    trainer_battle_opener: TrainerBattleOpener,
    
    grass: Texture,
    
    grass_active: bool,
    grass_x_offset: f32,
    grass_y_offset: f32,

}

static GRASS_X_OFFSET: f32 = 128.0 * 60.0; // width of image
static GRASS_Y_OFFSET: f32 = 47.0 * 60.0; // height of image

impl WildBattleOpener {

    pub fn new() -> Self {

        Self {

            //active: false,
            //finished: false,

            trainer_battle_opener: TrainerBattleOpener::new(),

            grass_active: true,
            grass_x_offset: GRASS_X_OFFSET,
            grass_y_offset: GRASS_Y_OFFSET,
            grass: crate::util::texture::byte_texture(include_bytes!("../../../../include/gui/battle/grass.png")),
        }

    }

}

impl Reset for WildBattleOpener {

    fn reset(&mut self) {
        self.grass_active = true;
        self.grass_x_offset = GRASS_X_OFFSET;
        self.grass_y_offset = GRASS_Y_OFFSET;
        self.trainer_battle_opener.reset();
    }
    
}

impl Load for WildBattleOpener {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        self.trainer_battle_opener.on_start();
    } 

}

impl Completable for WildBattleOpener {

    fn is_finished(&self) -> bool {
        return self.trainer_battle_opener.is_finished();
    }

}

impl Update for WildBattleOpener {

    fn update(&mut self, delta: f32) {
        self.trainer_battle_opener.update(delta);
        if self.grass_active {
            self.grass_x_offset -= 360.0 * delta;
            if self.grass_x_offset < 0.0 {
                self.grass_x_offset += GRASS_X_OFFSET * delta;
            }
            if self.trainer_battle_opener.offset() <= 130.0 && self.trainer_battle_opener.offset() % 2.0 > 1.0 {
                self.grass_y_offset -= 60.0 * delta;
            }
            if self.grass_y_offset <= 0.0 {
                self.grass_active = false;
            }
        }
        
    }

}

impl Render for WildBattleOpener {

    fn render(&self, tr: &TextRenderer) {
        self.trainer_battle_opener.render(tr);
    }

}

impl BattleOpener for WildBattleOpener {
    fn offset(&self) -> f32 {
        return self.trainer_battle_opener.offset();
    }

    fn render_below_panel(&self, _tr: &TextRenderer) {
        if self.grass_active {
            let y = 114.0 - self.grass_y_offset;
            draw(
                self.grass,
                self.grass_x_offset - GRASS_X_OFFSET,
                y,
            );
            draw(
                self.grass,
                self.grass_x_offset,
                y,
            );
            draw(
                self.grass,
                self.grass_x_offset + GRASS_X_OFFSET,
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

impl BattleTransition for WildBattleOpener {}