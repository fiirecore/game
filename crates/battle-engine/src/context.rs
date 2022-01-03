use pokedex::engine::{error::ImageError, graphics::Texture, Context};

pub struct BattleGuiData {
    pub panel: Texture,
    pub pokeball: Texture,
    pub smallui: Texture,
    pub padding: Texture,
    pub largeui: Texture,
    pub player: Texture,
    pub bar: Texture,
    pub ball: Texture,
}

impl BattleGuiData {
    pub fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            panel: Texture::new(ctx, include_bytes!("../assets/gui/panel.png"))?,
            pokeball: Texture::new(ctx, include_bytes!("../assets/thrown_pokeball.png"))?,
            smallui: Texture::new(ctx, include_bytes!("../assets/gui/small.png"))?,
            padding: Texture::new(ctx, include_bytes!("../assets/gui/padding.png"))?,
            largeui: Texture::new(ctx, include_bytes!("../assets/gui/large.png"))?,
            player: Texture::new(ctx, include_bytes!("../assets/player.png"))?,
            bar: Texture::new(ctx, include_bytes!("../assets/gui/bar.png"))?,
            ball: Texture::new(ctx, include_bytes!("../assets/gui/owned.png"))?,
        })
    }
}
