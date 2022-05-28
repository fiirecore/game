use pokengine::engine::graphics::{Draw, DrawImages, Texture};

use crate::context::BattleGuiData;

pub struct BattleBackground {
    background: Texture,
    ground: Texture,
    pub panel: Texture,
}

impl BattleBackground {
    pub fn new(gui: &BattleGuiData) -> Self {
        Self {
            background: gui.background.clone(),
            ground: gui.ground.clone(),
            panel: gui.panel.clone(),
        }
    }

    pub fn draw(&self, draw: &mut Draw, offset: f32) {
        draw.image(&self.background).position(0.0, 1.0);
        draw.image(&self.ground).position(113.0 - offset, 50.0);
        draw.image(&self.ground).position(offset, 103.0);
    }
}
