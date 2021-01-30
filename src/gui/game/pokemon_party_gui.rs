use crate::gui::gui::Background;
use crate::gui::gui::GuiComponent;
use crate::util::Load;

pub struct PokemonPartyGui {

    background: Background,

}

impl PokemonPartyGui {

    pub fn new() -> Self {
        Self {
            background: Background::new(crate::util::texture::byte_texture(include_bytes!("../../../build/assets/gui/game/party_panel.png")), 0.0, 0.0),
        }

    }

}

impl Load for PokemonPartyGui {

    fn load(&mut self) {
        self.background.load();
    }

}