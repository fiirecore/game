use std::sync::Arc;

use crate::{pokedex::pokemon::{owned::OwnedPokemon, PokemonTexture}, texture::PokemonTextures};

use engine::{
    notan::egui::{self, Pos2, Rect},
    utils::Entity,
    App,
};

mod select;

pub struct PartyGui {
    alive: bool,
    textures: Arc<PokemonTextures>,
    select: select::PartySelectMenu,
    swap: Option<usize>,
    accumulator: f32,
}

pub enum PartyAction {
    Select(usize),
}

impl PartyGui {
    pub fn new(textures: Arc<PokemonTextures>) -> Self {
        Self {
            alive: Default::default(),
            textures,
            select: Default::default(),
            accumulator: Default::default(),
            swap: Default::default(),
        }
    }

    pub fn ui(
        &mut self,
        app: &mut App,
        egui: &egui::Context,
        party: &mut [OwnedPokemon],
    ) -> Option<PartyAction> {
        if self.alive {
            self.accumulator += app.timer.delta_f32();
            if self.accumulator > 2.0 {
                self.accumulator -= 2.0;
            }

            if let Some(action) = self.select.ui(egui) {
                self.select.despawn();
                match action {
                    select::SelectAction::Select => {
                        return Some(PartyAction::Select(self.select.pokemon))
                    }
                    select::SelectAction::Summary => {
                        engine::log::warn!("todo: implement summary");
                    }
                    select::SelectAction::Swap => {
                        return None;
                    }
                }
            }

            return egui::Window::new("Party GUI")
                .show(egui, |ui| {
                    if let Some(action) = egui::Grid::new("Pokemon")
                        .show(ui, |ui| {
                            for (num, pokemon) in party.iter().enumerate() {
                                let (id, size) = self
                                    .textures
                                    .egui_id(&pokemon.pokemon.id, PokemonTexture::Icon)
                                    .unwrap_or((egui::TextureId::default(), (2.0, 2.0)));
                                let (a, b) = match self.accumulator > 1.0 {
                                    true => (0.0, 0.5),
                                    false => (0.5, 1.0),
                                };
                                if ui
                                    .add(egui::ImageButton::new(id, (size.0, size.0)).uv(Rect {
                                        min: Pos2 { x: 0.0, y: a },
                                        max: Pos2 { x: 1.0, y: b },
                                    }))
                                    .clicked()
                                {
                                    match self.swap {
                                        Some(swap) => {
                                            party.swap(swap, self.select.pokemon);
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

                                let mut level = [0u8; 8];

                                use std::io::Write;

                                if let Ok(()) =
                                    write!(&mut level as &mut [u8], "Lv. {}", pokemon.level)
                                {
                                    ui.label(match std::str::from_utf8(&level).ok() {
                                        Some(level) => level,
                                        None => "Lv. ?",
                                    });
                                }

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
}

impl Entity for PartyGui {
    fn spawn(&mut self) {
        self.alive = true;
        self.accumulator = 0.0;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
