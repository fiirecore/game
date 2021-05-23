use crate::{
    util::{Reset, Completable},
    graphics::draw_o_bottom,
    macroquad::prelude::Texture2D,
};

use crate::battle::{
    Battle,
    ui::transitions::BattleOpener,
};

use super::DefaultBattleOpener;

#[derive(Default)]
pub struct TrainerBattleOpener {
    opener: DefaultBattleOpener,
    trainer: Option<Texture2D>,
}

impl BattleOpener for TrainerBattleOpener {

    fn spawn(&mut self, battle: &Battle) {
        if let Some(trainer) = battle.data.trainer.as_ref() {
            self.trainer = Some(trainer.texture);
        }
    }

    fn update(&mut self, delta: f32) {
        self.opener.update(delta);
    }

    fn render_below_panel(&self, battle: &Battle) {
        draw_o_bottom(self.trainer, 144.0 - self.opener.offset, 74.0);
        self.opener.render_below_panel(battle);
    }

    fn render(&self) {
        self.opener.render();
    }

    fn offset(&self) -> f32 {
        self.opener.offset
    }

}

impl Reset for TrainerBattleOpener {

    fn reset(&mut self) {
        self.opener.reset();
        self.trainer = None;
    }

}

impl Completable for TrainerBattleOpener {
    fn finished(&self) -> bool {
        self.opener.finished()
    }
}