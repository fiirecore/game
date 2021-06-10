use crate::{
    util::{Reset, Completable}, 
    graphics::{byte_texture, position}, 
    tetra::{
        Context,
        math::Vec2,
        graphics::Texture,
    },
    battle_glue::BattleTrainerEntry,
};

use crate::battle::{
    ui::{
        view::ActiveRenderer,
        transitions::{
            BattleOpener,
            openers::DefaultBattleOpener,
        }
    },
};

pub struct WildBattleOpener {

    opener: DefaultBattleOpener,
    
    grass: Texture,
    offset: Vec2<f32>,

}

impl WildBattleOpener {
    const GRASS_WIDTH: f32 = 128.0;
    const GRASS_HEIGHT: f32 = 47.0;
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            opener: DefaultBattleOpener::new(ctx),
            grass: byte_texture(ctx, include_bytes!("../../../../../assets/battle/grass.png")),
            offset: Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT),
        }
    }
}

impl BattleOpener for WildBattleOpener {

    fn spawn(&mut self, _trainer: Option<&BattleTrainerEntry>) {}

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

    fn draw_below_panel(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        for active in opponent.iter() {
            active.renderer.draw(ctx, Vec2::new(-self.opener.offset, 0.0), crate::graphics::LIGHTGRAY);
        }
        self.opener.draw_below_panel(ctx, player, opponent);
        if self.offset.y > 0.0 {
            let y = 114.0 - self.offset.y;
            self.grass.draw(ctx, position(self.offset.x - Self::GRASS_WIDTH, y));
            self.grass.draw(ctx, position(self.offset.x, y));
            self.grass.draw(ctx, position(self.offset.x + Self::GRASS_WIDTH, y));
        }
    }

    fn draw(&self, ctx: &mut Context) {
        self.opener.draw(ctx);
    }
}

impl Reset for WildBattleOpener {
    fn reset(&mut self) {
        self.offset = Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT);
        self.opener.reset();
    }
}
impl Completable for WildBattleOpener {
    fn finished(&self) -> bool {
        self.opener.finished() && self.offset.y <= 0.0
    }
}