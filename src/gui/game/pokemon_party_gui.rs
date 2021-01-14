use crate::gui::gui::Panel;
use crate::gui::gui::GuiComponent;
use crate::util::traits::Loadable;

pub struct PokemonPartyGui {

    background: Panel,

}

impl PokemonPartyGui {

    pub fn new() -> Self {

        Self {

            background: Panel::new("gui/game/party_panel.png", 0, 0),

        }

    }

}

impl Loadable for PokemonPartyGui {

    fn load(&mut self) {
        self.background.load();
    }

}