use std::ops::Deref;

use battle::{party::PlayerParty, pokemon::PokemonIdentifier};

use pokengine::{
    engine::egui,
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::{owned::OwnedPokemon, Pokemon},
    },
};

use crate::{players::GuiRemotePlayers, BattleTrainer};

#[derive(Default)]
pub struct TargetPanel {
    alive: bool,
}

impl TargetPanel {
    pub fn spawn(&mut self) {
        self.alive = true;
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn ui<
        ID: Clone,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        egui: &egui::Context,
        local: &PlayerParty<ID, usize, OwnedPokemon<P, M, I>, BattleTrainer>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) -> Option<PokemonIdentifier<ID>> {
        if self.alive {
            egui::Window::new("Targets")
                .title_bar(false)
                .show(egui, |ui| {
                    let i = egui::Grid::new("TargetGrid")
                        .show(ui, |ui| {
                            if let Some(remote) =
                                remotes.players.get_index(remotes.current).map(|(.., r)| r)
                            {
                                for (index, pokemon) in remote.active_iter() {
                                    if ui
                                        .button(
                                            pokemon.as_ref().map(|p| p.name()).unwrap_or("Unknown"),
                                        )
                                        .clicked()
                                    {
                                        return Some(PokemonIdentifier(remote.id.clone(), index));
                                    }
                                }
                            }
                            ui.end_row();
                            for (index, pokemon) in local.active_iter() {
                                if ui.button(pokemon.name()).clicked() {
                                    return Some(PokemonIdentifier(local.id.clone(), index));
                                }
                            }
                            None
                        })
                        .inner;
                    if ui.button("Back").clicked() {
                        self.despawn();
                    }
                    i
                })
                .and_then(|i| i.inner)
                .flatten()
        } else {
            None
        }
    }
}
