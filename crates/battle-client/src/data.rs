use pokengine::engine::{graphics::Texture, notan::prelude::Graphics};
use serde::{Deserialize, Serialize};

pub type InitBattleGuiTextures = BattleGuiTextures<Texture>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleGuiTextures<T> {
    pub background: T,
    // pub panel: T,
    pub ground: T,
    pub pokeball: T,
    // pub smallui: T,
    // pub padding: T,
    // pub largeui: T,
    pub player: T,
    pub grass: T,
    pub bar: T,
    // pub ball: T,
}

impl BattleGuiTextures<Vec<u8>> {
    pub fn init(self, gfx: &mut Graphics) -> Result<InitBattleGuiTextures, String> {
        Ok(InitBattleGuiTextures {
            background: gfx.create_texture().from_image(&self.background).build()?,
            // panel: gfx
            //     .create_texture()
            //     .from_image(&self.panel)
            //     .build()?,
            ground: gfx.create_texture().from_image(&self.ground).build()?,
            pokeball: gfx.create_texture().from_image(&self.pokeball).build()?,
            // smallui: gfx
            //     .create_texture()
            //     .from_image(&self.smallui)
            //     .build()?,
            // padding: gfx
            //     .create_texture()
            //     .from_image(include_bytes!("../assets/gui/padding.png"))
            //     .build()?,
            // largeui: gfx
            //     .create_texture()
            //     .from_image(include_bytes!("../assets/gui/large.png"))
            //     .build()?,
            player: gfx.create_texture().from_image(&self.player).build()?,
            grass: gfx.create_texture().from_image(&self.grass).build()?,
            bar: gfx.create_texture().from_image(&self.bar).build()?,
            // ball: gfx.create_texture().from_image(&self.ball).build()?,
        })
    }
}
