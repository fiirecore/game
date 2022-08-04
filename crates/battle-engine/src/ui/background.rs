use pokengine::engine::{graphics::{Draw, DrawImages, Texture}, utils::HashMap};

/// Use a HashMap<BattleLocation, BattleBackground>
pub struct BattleBackground {
    // background: BattleBackgroundData,
    ground: Texture,
}

impl BattleBackground {
    pub fn new(textures: &crate::InitBattleGuiTextures) -> Self {

        /// To - do: create custom color palette backgrounds for battles based on battle location in a .ron file?
        /// 
        /// Still have the ground textures though

        Self {
            ground: textures.ground.clone(),
        }
    }

    pub fn draw_background(&self, draw: &mut Draw) {

    }

    pub fn draw(&self, draw: &mut Draw, offset: f32) {
        self.draw_background(draw);
        draw.image(&self.ground).position(113.0 - offset, 50.0);
        draw.image(&self.ground).position(offset, 103.0);
    }
}
