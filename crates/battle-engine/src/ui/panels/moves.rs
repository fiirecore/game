use pokengine::{engine::egui, pokedex::pokemon::owned::OwnedPokemon};

#[derive(Default)]
pub struct MovePanel {
    alive: bool,
}

pub enum ButtonState {
    Clicked(usize),
    Hovered(usize),
}

impl MovePanel {
    pub fn spawn(&mut self) {
        self.alive = true;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, pokemon: &OwnedPokemon) -> Option<ButtonState> {
        let per_row = (pokemon.moves.len() as f32).sqrt().ceil() as usize;
        let i = egui::Grid::new("Move Grid")
            .show(ui, |ui| {
                let mut b = None;
                for (i, m) in pokemon.moves.iter().enumerate() {
                    let button = ui.button(&m.0.name);
                    if button.clicked() {
                        b = Some(ButtonState::Clicked(i));
                    } else if button.hovered() {
                        b = Some(ButtonState::Hovered(i));
                    }
                    if i % per_row == per_row - 1 {
                        ui.end_row();
                    }
                }
                b
            })
            .inner;
        if ui.button("Back").clicked() {
            self.despawn();
        }
        i
    }
}
