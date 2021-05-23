use crate::{
    util::{Reset, Completable, WIDTH, HEIGHT},
    gui::DynamicText,
    macroquad::prelude::{Color, BLACK, draw_rectangle},
};

use crate::battle::{
    Battle,
    ui::transitions::BattleCloser,
};

pub struct WildBattleCloser {
    color: Color,
    world: bool,
}

impl Default for WildBattleCloser {
    fn default() -> Self {
        Self {
            color: BLACK,
            world: false,
        }
    }
}

impl BattleCloser for WildBattleCloser {

    fn spawn(&mut self, _battle: &Battle, _text: &mut DynamicText) {}

    fn update(&mut self, delta: f32, _text: &mut DynamicText) {
        if self.world {
            self.color.a -= 4.5 * delta;
        } else {
            self.color.a += 4.5 * delta;
        }
        if self.color.a >= 1.0 {
            self.world = true;
        }
    }

    fn world_active(&self) -> bool {
        self.world
    }

    fn render(&self) {
        draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, self.color);
    }

    fn render_battle(&self) {}

}

impl Reset for WildBattleCloser {
    fn reset(&mut self) {
        self.color.a = 0.0;
        self.world = false;
    }    
}

impl Completable for WildBattleCloser {
    fn finished(&self) -> bool {
        self.color.a <= 0.0 && self.world
    }
}