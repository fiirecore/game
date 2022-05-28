use std::ops::Deref;

use crate::pokedex::{
    item::Item,
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
};

use engine::{
    notan::egui::{self, Pos2, Rect},
    utils::Entity,
    App,
};

use crate::{texture::PokemonTexture, PokedexClientData};

mod select;

pub struct PartyGui<D: Deref<Target = PokedexClientData>> {
    alive: bool,
    data: D,
    select: select::PartySelectMenu,
    swap: Option<usize>,
    accumulator: f32,
}

pub enum PartyAction {
    Select(usize),
}

impl<D: Deref<Target = PokedexClientData>> PartyGui<D> {
    pub fn new(data: D) -> Self {
        Self {
            alive: Default::default(),
            data,
            select: Default::default(),
            accumulator: Default::default(),
            swap: Default::default(),
        }
    }

    pub fn ui<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        egui: &egui::Context,
        party: &mut [OwnedPokemon<P, M, I>],
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
                                    .data
                                    .pokemon_textures
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

impl<D: Deref<Target = PokedexClientData>> Entity for PartyGui<D> {
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
