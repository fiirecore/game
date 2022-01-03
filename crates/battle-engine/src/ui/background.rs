use pokedex::engine::{graphics::Texture, Context};

use crate::context::BattleGuiData;

pub struct BattleBackground {
    background: Texture,
    ground: Texture,
    pub panel: Texture,
}

impl BattleBackground {
    pub fn new(ctx: &mut Context, gui: &BattleGuiData) -> Self {
        Self {
            background: Texture::new(ctx, include_bytes!("../../assets/background.png")).unwrap(),
            ground: Texture::new(ctx, include_bytes!("../../assets/ground.png")).unwrap(),
            panel: gui.panel.clone(),
        }
    }

    pub fn draw(&self, ctx: &mut Context, offset: f32) {
        self.background.draw(ctx, 0.0, 1.0, Default::default());
        self.ground
            .draw(ctx, 113.0 - offset, 50.0, Default::default());
        self.ground.draw(ctx, offset, 103.0, Default::default());
    }
}
