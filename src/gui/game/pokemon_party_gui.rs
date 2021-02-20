use crate::gui::background::Background;

pub struct PokemonPartyGui {

    background: Background,

}

impl PokemonPartyGui {

    pub fn new() -> Self {
        Self {
            background: Background::new(crate::util::graphics::texture::byte_texture(include_bytes!("../../../build/assets/gui/game/party_panel.png")), 0.0, 0.0),
        }

    }

}