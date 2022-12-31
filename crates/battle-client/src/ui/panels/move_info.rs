use std::io::Write;

use pokengine::{engine::bevy_egui::egui, pokedex::moves::owned::OwnedMove};

pub struct MoveInfoPanel;

impl MoveInfoPanel {
    pub fn ui(ui: &mut egui::Ui, m: &OwnedMove) {
        egui::Grid::new("MoveInfoGrid").show(ui, |ui| {
            let mut pp = [0u8; 8];
            if let Ok(()) = write!(&mut pp as &mut [u8], "PP {}/{}", m.pp(), m.0.pp) {
                if let Ok(str) = std::str::from_utf8(&pp) {
                    ui.label(str);
                }
            }
            ui.end_row();
            let mut move_type = [0u8; 16];
            if let Ok(()) = write!(&mut move_type as &mut [u8], "TYPE/{:?}", m.0.pokemon_type) {
                if let Ok(str) = std::str::from_utf8(&move_type) {
                    ui.label(str);
                }
            }
        });
    }
}
