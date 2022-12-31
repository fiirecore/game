use crate::{
    pokedex::pokemon::{owned::OwnedPokemon, data::PokemonTexture},
    texture::PokemonTextures,
};

use engine::bevy_egui::egui::{self, Pos2, Rect};

mod select;
mod summary;

#[derive(Default)]
pub struct PartyGui {
    alive: bool,
    select: select::PartySelectMenu,
    summary: summary::SummaryGui,
    accumulator: f32,
    swap: Option<usize>,
}

pub enum PartyAction {
    Select(usize),
}

impl PartyGui {

    pub fn ui(
        &mut self,
        textures: &PokemonTextures,
        egui: &egui::Context,
        delta: f32,
        party: &mut [OwnedPokemon],
    ) -> Option<PartyAction> {
        if self.alive() {
            self.accumulator += delta;
            if self.accumulator > 2.0 {
                self.accumulator -= 2.0;
            }

            if let Some(action) = self.select.ui(egui) {
                self.select.despawn();
                let pokemon = self.select.pokemon;
                match action {
                    select::SelectAction::Select => return Some(PartyAction::Select(pokemon)),
                    select::SelectAction::Summary => {
                        self.summary.spawn(pokemon);
                    }
                    select::SelectAction::Swap => {
                        self.swap = Some(pokemon);
                    }
                }
            }

            if let Some(()) = self.summary.ui(textures, egui, party) {
                self.summary.despawn();
            }

            return egui::Window::new("Party GUI")
                .show(egui, |ui| {
                    if let Some(action) = egui::Grid::new("Pokemon")
                        .show(ui, |ui| {
                            for (num, pokemon) in party.iter().enumerate() {
                                let id = textures
                                    .get_egui(&pokemon.pokemon.id, PokemonTexture::Icon)
                                    .unwrap_or_default();
                                let (a, b) = match self.accumulator > 1.0 {
                                    true => (0.0, 0.5),
                                    false => (0.5, 1.0),
                                };
                                if ui
                                    .add(egui::ImageButton::new(id, PokemonTextures::ICON_SIZE).uv(
                                        Rect {
                                            min: Pos2 { x: 0.0, y: a },
                                            max: Pos2 { x: 1.0, y: b },
                                        },
                                    ))
                                    .clicked()
                                {
                                    match self.swap.take() {
                                        Some(swap) => {
                                            party.swap(swap, num);
                                            return None;
                                        }
                                        None => {
                                            if !self.select.alive() {
                                                self.select.spawn(num);
                                            }
                                        }
                                    }
                                }

                                ui.label(pokemon.name());

                                ui.label(format!("Lv. {}", pokemon.level));

                                ui.end_row();
                            }
                            None
                        })
                        .inner
                    {
                        return Some(action);
                    }
                    if ui.button("Close").clicked() {
                        self.despawn();
                    }
                    None
                })
                .and_then(|i| i.inner)
                .flatten();
        }
        None
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        self.accumulator = 0.0;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }
}
