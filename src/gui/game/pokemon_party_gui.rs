use crate::gui::gui::Background;
use crate::gui::gui::GuiComponent;
use crate::util::Load;

pub struct PokemonPartyGui {

    background: Background,

}

impl PokemonPartyGui {

    pub async fn new() -> Self {
        Self {
            background: Background::new(crate::util::texture::load_texture(crate::util::file::asset_as_pathbuf("gui/game/party_panel.png")).await, 0.0, 0.0),
        }

    }

}

impl Load for PokemonPartyGui {

    fn load(&mut self) {
        self.background.load();
    }

}