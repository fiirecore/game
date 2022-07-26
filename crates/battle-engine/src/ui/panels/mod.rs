use core::ops::Deref;

use battle::{moves::BattleMove, pokemon::Indexed, prelude::BattleType};
use pokengine::{
    engine::{egui, utils::Entity, App},
    gui::{
        bag::{BagAction, BagGui},
        party::{PartyAction, PartyGui},
    },
    pokedex::{
        item::{Item, ItemId},
        moves::Move,
        pokemon::Pokemon,
    },
    PokedexClientData,
};

pub mod move_info;
pub mod moves;
pub mod target;

pub mod level;

use self::{
    move_info::MoveInfoPanel,
    moves::{ButtonState, MovePanel},
    target::TargetPanel,
};

use crate::players::{GuiLocalPlayer, GuiRemotePlayers};

pub struct BattlePanel<D: Deref<Target = PokedexClientData> + Clone> {
    state: BattleOptions,
    moves: MovePanel,
    targets: TargetPanel,
    party: PartyGui<D>,
    bag: BagGui<D>,
}

pub enum BattleOptions {
    NotAlive,
    Main,
    Fight,
    Target(usize, BattleTargetOption),
    Bag,
    Pokemon,
    PartyOnly,
}

#[derive(Clone, Copy)]
pub enum BattleTargetOption {
    Pokemon(usize),
    Item(ItemId),
}

pub enum BattleAction<ID> {
    Action(BattleMove<ID>),
    Forfeit,
}

impl<D: Deref<Target = PokedexClientData> + Clone> BattlePanel<D> {
    pub fn new(data: D) -> Self {
        Self {
            state: BattleOptions::NotAlive,
            moves: MovePanel::default(),
            targets: TargetPanel::default(),
            party: PartyGui::new(data.clone()),
            bag: BagGui::new(data),
        }
    }

    pub fn spawn(&mut self, party_only: bool) {
        self.reset();
        self.state = match party_only {
            true => BattleOptions::PartyOnly,
            false => BattleOptions::Main,
        };
    }

    pub fn despawn(&mut self) {
        self.state = BattleOptions::NotAlive;
    }

    pub fn alive(&self) -> bool {
        !matches!(self.state, BattleOptions::NotAlive)
    }

    pub fn reset(&mut self) {
        self.state = BattleOptions::NotAlive;
    }

    pub fn ui<
        ID: Clone,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        egui: &egui::Context,
        local: &mut GuiLocalPlayer<ID, P, M, I>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) -> Option<BattleAction<ID>> {
        match self.state {
            BattleOptions::Fight => {
                match local
                    .selecting
                    .as_ref()
                    .and_then(|r| local.player.active(r.start).map(|p| (r.start, p)))
                {
                    Some((active, pokemon)) => {
                        if self.moves.alive() {
                            return egui::Window::new("Moves")
                                .title_bar(false)
                                .show(egui, |ui| {
                                    egui::Grid::new("MoveGrid")
                                        .show(ui, |ui| {
                                            if let Some(response) = self.moves.ui(ui, pokemon) {
                                                match response {
                                                    ButtonState::Clicked(i)
                                                    | ButtonState::Hovered(i) => {
                                                        if let Some(m) = pokemon.moves.get(i) {
                                                            MoveInfoPanel::ui(ui, m);
                                                        }
                                                    }
                                                }
                                                if let ButtonState::Clicked(i) = response {
                                                    match pokemon.moves[i].0.target.needs_input() {
                                                        true => {
                                                            self.state = BattleOptions::Target(
                                                                active,
                                                                BattleTargetOption::Pokemon(i),
                                                            );
                                                            self.targets.spawn();
                                                        }
                                                        false => {
                                                            // println!("untargeted");
                                                            return Some(
                                                                BattleAction::<ID>::Action(
                                                                    BattleMove::Move(i, None),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                    // return Some(BattleAction::Move(i, None));
                                                }
                                            }
                                            None
                                        })
                                        .inner
                                })
                                .and_then(|i| i.inner)
                                .flatten();
                        } else {
                            self.state = BattleOptions::Main;
                        }
                    }
                    None => {
                        self.state = BattleOptions::Main;
                    }
                }
                None
            }
            BattleOptions::Target(_, option) => {
                if self.targets.alive() {
                    if let Some(id) = self.targets.ui(egui, &local.player, remotes) {
                        self.state = BattleOptions::Main;
                        return Some(BattleAction::Action(match option {
                            BattleTargetOption::Pokemon(index) => BattleMove::Move(index, Some(id)),
                            BattleTargetOption::Item(item) => {
                                if let Some(item) = local.bag.get_mut(&item) {
                                    item.count = item.count.saturating_sub(1);
                                }
                                BattleMove::UseItem(Indexed(id, item))
                            }
                        }));
                    }
                } else {
                    self.state = match &option {
                        BattleTargetOption::Pokemon(..) => BattleOptions::Fight,
                        BattleTargetOption::Item(..) => BattleOptions::Bag,
                    };
                }
                None
            }
            BattleOptions::Bag => {
                if self.bag.alive() {
                    if let Some(action) = self.bag.ui(egui, &mut local.bag) {
                        match action {
                            BagAction::Use(item) => {
                                if let Some(active) = local.selecting.as_ref().map(|r| r.start) {
                                    self.state = BattleOptions::Target(
                                        active,
                                        BattleTargetOption::Item(item),
                                    );
                                    self.targets.spawn();
                                }
                            }
                        }
                    }
                } else {
                    self.state = BattleOptions::Main;
                }
                None
            }
            BattleOptions::Pokemon => {
                if self.party.alive() {
                    if let Some(action) = self.party.ui(app, egui, &mut local.player.pokemon) {
                        match action {
                            PartyAction::Select(p) => {
                                if Some(p) != local.selecting.as_ref().map(|r| r.start) {
                                    return Some(BattleAction::Action(BattleMove::Switch(p)));
                                }
                            }
                        }
                    }
                } else {
                    self.state = BattleOptions::Main;
                }
                None
            }
            BattleOptions::NotAlive => None,
            BattleOptions::Main => egui::Window::new("Battle")
                .title_bar(false)
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0; 2])
                .show(egui, |ui| {
                    egui::Grid::new("BattlePanel")
                        .show(ui, |ui| {
                            if let Some(pokemon) = local
                                .selecting
                                .as_ref()
                                .and_then(|r| local.player.active(r.start))
                            {
                                ui.label(format!("What will {} do?", pokemon.name()));
                            }

                            egui::Grid::new("BattleOptions")
                                .show(ui, |ui| {
                                    if ui.button("Fight").clicked() {
                                        self.state = BattleOptions::Fight;
                                        self.moves.spawn();
                                    }
                                    if ui.button("Bag").clicked() {
                                        self.state = BattleOptions::Bag;
                                        self.bag.spawn();
                                    }
                                    ui.end_row();
                                    if ui.button("Pokemon").clicked() {
                                        self.state = BattleOptions::Pokemon;
                                        self.party.spawn();
                                    }
                                    if ui
                                        .button(match local.data.type_ {
                                            BattleType::Wild => "Run",
                                            BattleType::Trainer | BattleType::GymLeader => {
                                                "Forfeit"
                                            }
                                        })
                                        .clicked()
                                    {
                                        return Some(BattleAction::Forfeit);
                                    }
                                    None
                                })
                                .inner
                        })
                        .inner
                })
                .and_then(|i| i.inner)
                .flatten(),
            BattleOptions::PartyOnly => {
                if self.party.alive() {
                    if let Some(action) = self.party.ui(app, egui, &mut local.player.pokemon) {
                        match action {
                            PartyAction::Select(p) => {
                                if Some(p) != local.selecting.as_ref().map(|r| r.start) {
                                    return Some(BattleAction::Action(BattleMove::Switch(p)));
                                }
                            }
                        }
                    }
                } else {
                    self.state = BattleOptions::NotAlive;
                }
                None
            }
        }

        // Panel::draw(app, plugins, 120.0, 113.0, 120.0, 47.0);

        // draw_text_left(
        //     ctx,
        //     eng,
        //     &1,
        //     "What will",
        //     11.0,
        //     123.0,
        //     DrawParams::color(TextColor::WHITE),
        // );
        // draw_text_left(
        //     ctx,
        //     eng,
        //     &1,
        //     &self.pokemon_do,
        //     11.0,
        //     139.0,
        //     DrawParams::color(TextColor::WHITE),
        // );

        // for (index, string) in self.buttons.iter().enumerate() {
        //     draw_text_left(
        //         ctx,
        //         eng,
        //         &0,
        //         string,
        //         138.0 + if index % 2 == 0 { 0.0 } else { 56.0 },
        //         123.0 + if index >> 1 == 0 { 0.0 } else { 16.0 },
        //         DrawParams::color(TextColor::BLACK),
        //     )
        // }

        // draw_cursor(
        //     ctx,
        //     eng,
        //     131.0 + if self.cursor % 2 == 0 { 0.0 } else { 56.0 },
        //     126.0 + if (self.cursor >> 1) == 0 { 0.0 } else { 16.0 },
        //     Default::default(),
        // );
    }
}
