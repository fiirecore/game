use crate::pokedex::PokedexClientData;

use crate::{
    engine::{
        graphics::{draw_rectangle, Color},
        gui::MessageBox,
        utils::{Completable, Reset, HEIGHT, WIDTH},
        Context,
    },
    game::battle_glue::{BattleId, BattleTrainerEntry},
};

use crate::battle::manager::transitions::BattleCloser;

pub struct WildBattleCloser {
    color: Color,
    world: bool,
}

impl Default for WildBattleCloser {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(0, 0, 0, 255),
            world: false,
        }
    }
}

impl BattleCloser for WildBattleCloser {
    fn spawn<'d>(
        &mut self,
        _: &PokedexClientData,
        _: &BattleId,
        _: &str,
        _: Option<&BattleId>,
        // _: Option<&TrainerData>,
        _: Option<&BattleTrainerEntry>,
        _text: &mut MessageBox,
    ) {
    }

    fn update(&mut self, _ctx: &mut Context, delta: f32, _text: &mut MessageBox) {
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
