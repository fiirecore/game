use std::cell::Cell;

use crate::engine::{egui, App, Plugins};

#[derive(Debug, Default)]
pub struct Settings(Cell<bool>);

impl Settings {
    pub fn spawn(&self) {
        self.0.set(true);
        // self.select_text = Some(BATTLE_OPTIONS));
    }

    pub fn despawn(&self) {
        self.0.set(false);
        // self.items.clear()
    }

    pub fn alive(&self) -> bool {
        self.0.get()
    }

    pub fn ui(&self, app: &mut App, plugins: &mut Plugins, egui: &egui::Context) {
        if self.alive() {
            egui::Window::new("Settings").show(egui, |ui| {
                if let Some(mut audio) = plugins.get_mut::<crate::engine::AudioContext>() {
                    if ui.add(egui::Slider::new(&mut audio.volume, 0.0..=1.0).text("Volume")).changed() {
                        audio.update_volume(app);
                    }
                }
                if ui.button("Exit").clicked() {
                    self.despawn();
                }
            });
        }
    }
}
