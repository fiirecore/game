use game::{
    util::{Reset, Completable},
    macroquad::prelude::{Vec2, Texture2D},
    graphics::{byte_texture, draw},
};

use crate::{
    Battle,
    ui::transitions::{
        BattleOpener,
        openers::DefaultBattleOpener,
    }
};

pub struct WildBattleOpener {

    opener: DefaultBattleOpener,
    
    grass: Texture2D,
    offset: Vec2,

}

impl WildBattleOpener {
    const GRASS_WIDTH: f32 = 128.0;
    const GRASS_HEIGHT: f32 = 47.0;
}

impl Default for WildBattleOpener {
    fn default() -> Self {
        Self {
            opener: DefaultBattleOpener::default(),
            grass: byte_texture(include_bytes!("../../../../assets/grass.png")),
            offset: Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT),
        }
    }
}

impl BattleOpener for WildBattleOpener {

    fn spawn(&mut self, _battle: &Battle) {}

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

    fn offset(&self) -> f32 {
        self.opener.offset()
    }

    fn render_below_panel(&self, battle: &Battle) {
        self.opener.render_below_panel(battle);
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
        for active in battle.opponent.active.iter() {
            active.renderer.render(Vec2::new(-self.opener.offset, 0.0));
        }
    }

    fn render(&self) {
        self.opener.render();
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