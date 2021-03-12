use crate::util::graphics::Texture;
use firecore_util::Entity;
use crate::battle::transitions::BattleOpener;
use crate::battle::transitions::BattleTransition;
use crate::util::graphics::draw;
use crate::util::{Reset, Completable};
use super::trainer_battle_opener::TrainerBattleOpener;
pub struct WildBattleOpener {

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

            trainer_battle_opener: TrainerBattleOpener::new(),

            grass_active: true,
            grass_x_offset: GRASS_X_OFFSET,
            grass_y_offset: GRASS_Y_OFFSET,
            grass: crate::util::graphics::texture::byte_texture(include_bytes!("../../../../build/assets/gui/battle/grass.png")),
        }

    }

}

impl BattleTransition for WildBattleOpener {
    
    fn on_start(&mut self) {
        self.trainer_battle_opener.on_start();
    }

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

    fn render(&self) {
        self.trainer_battle_opener.render();
    }

}

impl BattleOpener for WildBattleOpener {
    fn offset(&self) -> f32 {
        return self.trainer_battle_opener.offset();
    }

    fn render_below_panel(&self) {
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

impl Reset for WildBattleOpener {

    fn reset(&mut self) {
        self.grass_active = true;
        self.grass_x_offset = GRASS_X_OFFSET;
        self.grass_y_offset = GRASS_Y_OFFSET;
        self.trainer_battle_opener.reset();
    }
    
}

impl Completable for WildBattleOpener {

    fn is_finished(&self) -> bool {
        return self.trainer_battle_opener.is_finished();
    }

}

impl Entity for WildBattleOpener {
    fn spawn(&mut self) {
        self.reset();
        self.trainer_battle_opener.spawn();
    }

    fn despawn(&mut self) {
        self.trainer_battle_opener.despawn();
    }

    fn is_alive(&self) -> bool {
        return self.trainer_battle_opener.is_alive();
    }
}