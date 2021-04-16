use game::{
    util::{Entity, Reset, Completable},
    macroquad::prelude::{Vec2, Texture2D},
    graphics::{byte_texture, draw},
};

use crate::transitions::{BattleTransition, BattleOpener};

use super::trainer::TrainerBattleOpener;
pub struct WildBattleOpener {

    opener: TrainerBattleOpener,
    
    grass: Texture2D,
    offset: Vec2,

}

impl WildBattleOpener {
    
    const GRASS_WIDTH: f32 = 128.0;
    const GRASS_HEIGHT: f32 = 47.0;

    pub fn new() -> Self {
        Self {
            opener: TrainerBattleOpener::new(),

            grass: byte_texture(include_bytes!("../../../assets/grass.png")),
            offset: Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT),
        }
    }

}

impl BattleTransition for WildBattleOpener {
    
    fn on_start(&mut self) {
        self.opener.on_start();
    }

    fn update(&mut self, delta: f32) {
        self.opener.update(delta);
        if self.offset.y > 0.0 {
            self.offset.x -= 360.0 * delta;
            if self.offset.x < 0.0 {
                self.offset.x += Self::GRASS_WIDTH;
            }
            if self.opener.offset() <= 130.0 {
                self.offset.y -= 60.0 * delta;
            }
        }
        
    }

    fn render(&self) {
        self.opener.render();
    }

}

impl BattleOpener for WildBattleOpener {
    fn offset(&self) -> f32 {
        self.opener.offset()
    }

    fn render_below_panel(&self) {
        if self.offset.y > 0.0 {
            let y = 114.0 - self.offset.y;
            draw(
                self.grass,
                self.offset.x - Self::GRASS_WIDTH,
                y,
            );
            draw(
                self.grass,
                self.offset.x,
                y,
            );
            draw(
                self.grass,
                self.offset.x + Self::GRASS_WIDTH,
                y,
            );
        }
    }
}

impl Reset for WildBattleOpener {

    fn reset(&mut self) {
        self.offset = Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT);
        self.opener.reset();
    }
    
}

impl Completable for WildBattleOpener {

    fn is_finished(&self) -> bool {
        self.opener.is_finished() && self.offset.y <= 0.0
    }

}

impl Entity for WildBattleOpener {
    fn spawn(&mut self) {
        self.opener.spawn();
        self.reset();
    }

    fn despawn(&mut self) {
        self.opener.despawn();
    }

    fn is_alive(&self) -> bool {
        self.opener.is_alive()
    }
}