use pokengine::engine::{graphics::Texture, notan::prelude::Graphics};

#[derive(Clone)]
pub struct BattleGuiData {
    pub background: Texture,
    pub panel: Texture,
    pub ground: Texture,
    pub pokeball: Texture,
    pub smallui: Texture,
    pub padding: Texture,
    pub largeui: Texture,
    pub player: Texture,
    pub grass: Texture,
    pub bar: Texture,
    pub ball: Texture,
}

impl BattleGuiData {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            background: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/background.png"))
                .build()?,
            panel: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/panel.png"))
                .build()?,
            ground: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/ground.png"))
                .build()?,
            pokeball: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/thrown_pokeball.png"))
                .build()?,
            smallui: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/small.png"))
                .build()?,
            padding: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/padding.png"))
                .build()?,
            largeui: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/large.png"))
                .build()?,
            player: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/player.png"))
                .build()?,
            grass: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/grass.png"))
                .build()?,
            bar: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/bar.png"))
                .build()?,
            ball: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/gui/owned.png"))
                .build()?,
        })
    }
}
