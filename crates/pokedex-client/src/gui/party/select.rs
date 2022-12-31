use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use engine::bevy_egui::egui;

#[derive(Default)]
pub struct PartySelectMenu {
    alive: bool,
    pub pokemon: usize,
}

pub enum SelectAction {
    Select,
    Summary,
    Swap,
    // Item,
    // Cancel,
}

impl PartySelectMenu {
    pub fn spawn(&mut self, pokemon: usize) {
        self.alive = true;
        self.pokemon = pokemon;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn ui(&self, egui: &egui::Context) -> Option<SelectAction> {
        self.alive().then(|| {
            egui::Window::new("Select")
                .title_bar(false)
                .show(egui, |ui| {
                    if ui.button("Select").clicked() {
                        return Some(SelectAction::Select);
                    }
                    if ui.button("Summary").clicked() {
                        return Some(SelectAction::Summary);
                    }
                    if ui.button("Swap").clicked() {
                        return Some(SelectAction::Swap);
                    }
                    if ui.button("Give").clicked() {}
                    None
                })
                .and_then(|i| i.inner)
                .flatten()
        }).flatten()
    }
    
}
