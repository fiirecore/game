use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::{
    pokedex::pokemon::{owned::OwnedPokemon, PokemonTexture},
    texture::PokemonTextures,
};

use engine::egui::{self, Pos2, Rect};

mod select;
mod summary;

pub struct PartyGui {
    alive: AtomicBool,
    select: select::PartySelectMenu,
    summary: summary::SummaryGui,
    swap: Mutex<Option<usize>>,
    accumulator: Mutex<f32>,
    textures: Arc<PokemonTextures>,
}

pub enum PartyAction {
    Select(usize),
}

impl PartyGui {
    pub fn new(textures: Arc<PokemonTextures>) -> Self {
        Self {
            alive: Default::default(),
            select: Default::default(),
            accumulator: Default::default(),
            swap: Default::default(),
            summary: summary::SummaryGui::new(textures.clone()),
            textures,
        }
    }

    pub fn ui(
        &self,
        egui: &egui::Context,
        party: &mut [OwnedPokemon],
        delta: f32,
    ) -> Option<PartyAction> {
        if self.alive() {
            if let Ok(mut accumulator) = self.accumulator.try_lock() {
                *accumulator += delta;
                if *accumulator > 2.0 {
                    *accumulator -= 2.0;
                }
            }

            if let Some(action) = self.select.ui(egui) {
                self.select.despawn();
                let pokemon = self.select.pokemon.load(Ordering::Relaxed);
                match action {
                    select::SelectAction::Select => return Some(PartyAction::Select(pokemon)),
                    select::SelectAction::Summary => {
                        self.summary.spawn(pokemon);
                    }
                    select::SelectAction::Swap => {
                        if let Ok(mut swap) = self.swap.try_lock() {
                            *swap = Some(pokemon);
                        }
                    }
                }
            }

            if let Some(()) = self.summary.ui(egui, party) {
                self.summary.despawn();
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
                                let (a, b) = match self
                                    .accumulator
                                    .try_lock()
                                    .as_deref()
                                    .copied()
                                    .unwrap_or_default()
                                    > 1.0
                                {
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
                                    if let Ok(mut swap) = self.swap.try_lock() {
                                        match swap.take() {
                                            Some(swap) => {
                                                party.swap(
                                                    swap,
                                                    num,
                                                );
                                                return None;
                                            }
                                            None => {
                                                if !self.select.alive() {
                                                    self.select.spawn(num);
                                                }
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

    pub fn spawn(&self) {
        self.alive.store(true, Ordering::Relaxed);
        if let Ok(mut accumulator) = self.accumulator.try_lock() {
            *accumulator = 0.0;
        }
    }

    pub fn despawn(&self) {
        self.alive.store(false, Ordering::Relaxed)
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }
}
