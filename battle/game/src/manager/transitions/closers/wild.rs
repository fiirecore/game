use game::{
    util::{Reset, Completable, WIDTH, HEIGHT},
    gui::TextDisplay,
    graphics::draw_rectangle,
    pokedex::trainer::{TrainerData, TrainerId},
    battle_glue::BattleTrainerEntry,
    tetra::{
        Context,
        graphics::Color,
    },
};

use crate::manager::transitions::BattleCloser;

pub struct WildBattleCloser {
    color: Color,
    world: bool,
}

impl Default for WildBattleCloser {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            world: false,
        }
    }
}

impl BattleCloser for WildBattleCloser {

    fn spawn(&mut self, _: Option<&TrainerId>, _: Option<&TrainerData>, _: Option<&BattleTrainerEntry>, _text: &mut TextDisplay) {}

    fn update(&mut self, _ctx: &mut Context, delta: f32, _text: &mut TextDisplay) {
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

    fn draw(&self, ctx: &mut Context) {
        draw_rectangle(ctx, 0.0, 0.0, WIDTH, HEIGHT, self.color);
    }

    fn draw_battle(&self, _ctx: &mut Context) {}

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