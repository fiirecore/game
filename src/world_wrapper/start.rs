use std::{sync::Arc, rc::Rc};

use firecore_battle_client::pokengine::texture::{PokemonTextures, ItemTextures};

use crate::{pokedex::trainer::InitTrainer, settings::Settings};

use crate::{
    pokengine::{
        gui::{bag::BagGui, party::PartyGui},
    },
};

use crate::engine::{
    controls::{pressed, Control},
    notan::egui,
    App, Plugins,
};

pub struct StartMenu {
    alive: bool,
    cursor: usize,
    settings: Rc<Settings>,
    party: PartyGui,
    bag: BagGui,
}

impl StartMenu {
    pub(crate) fn new(settings: Rc<Settings>, pokemon: Arc<PokemonTextures>, items: Arc<ItemTextures>) -> Self {
        Self {
            alive: false,
            cursor: 0,
            settings,
            party: PartyGui::new(pokemon),
            bag: BagGui::new(items),
        }
    }

    pub fn ui(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        egui: &egui::Context,
        user: &mut InitTrainer,
    ) -> Option<super::WorldRequest> {
        if pressed(app, plugins, Control::Start) {
            self.alive = !self.alive;
        }
        self.settings.ui(app, plugins, egui);
        self.bag.ui(egui, &mut user.bag);
        self.party.ui(egui, &mut user.party, app.timer.delta_f32());
        match self.alive {
            true => egui::Window::new("Menu")
                .title_bar(false)
                .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
                .show(egui, |ui| {
                    if ui.button("Save").clicked() {
                        return Some(super::WorldRequest::Save);
                    }
                    if ui.button("Settings").clicked() {
                        self.settings.spawn();
                    }
                    if ui.button("Bag").clicked() {
                        self.bag.spawn();
                    }
                    if ui.button("Party").clicked() {
                        self.party.spawn();
                    }
                    if ui.button("Exit to Menu").clicked() {
                        return Some(super::WorldRequest::Exit);
                    }
                    if ui.button("Close").clicked() {
                        self.alive = false;
                    }
                    None
                }),
            false => egui::Window::new("Menu Button")
                .title_bar(false)
                .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
                .show(egui, |ui| {
                    if ui.button("Menu").clicked() {
                        self.alive = true;
                    }
                    None
                }),
        }
        .and_then(|i| i.inner)
        .flatten()
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        self.cursor = 0;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

}
