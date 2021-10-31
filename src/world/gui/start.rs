use engine::{
    gui::Panel,
    input::{pressed, Control},
    tetra::math::Vec2,
    util::Entity,
    EngineContext,
};
use pokedex::{
    context::PokedexClientContext,
    gui::{bag::BagGui, party::PartyGui},
};
use saves::PlayerData;
use std::rc::Rc;

use crate::{
    game::quit,
    state::{Action, MainStates},
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2<f32>,
    buttons: [&'static str; 6],
    cursor: usize,
    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
    // world_map: WorldMapGui,
}

impl StartMenu {
    pub fn new(party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            alive: false,
            pos: Vec2::new(169.0, 1.0),
            buttons: ["Save", "Bag", "Pokemon", "Menu", "Exit", "Cancel"],
            cursor: 0,
            party,
            bag,
        }
    }

    pub fn update<'d>(
        &mut self,
        ctx: &EngineContext,
        dex: &PokedexClientContext<'d>,
        save: &mut PlayerData<'d>,
        delta: f32,
        input_lock: bool,
        action: &mut Option<Action>,
    ) {
        if self.bag.alive() && !input_lock {
            self.bag.input(ctx, &save.bag.items);
            // bag_gui.up
        } else if self.party.alive() {
            if !input_lock {
                self.party.input(ctx, dex, save.party.as_mut_slice());
            }
            self.party.update(delta);
        } else if !input_lock {
            if pressed(ctx, Control::B) || pressed(ctx, Control::Start) {
                self.despawn();
            }

            if pressed(ctx, Control::A) {
                match self.cursor {
                    0 => {
                        // Save
                        save.should_save = true;
                    }
                    1 => {
                        // Bag
                        self.bag.spawn();
                    }
                    2 => {
                        // Pokemon
                        spawn_party(dex, &self.party, save);
                    }
                    3 => {
                        // Exit to Main Menu
                        *action = Some(Action::Goto(MainStates::Menu));
                        self.despawn();
                    }
                    4 => {
                        // Exit Game
                        quit();
                    }
                    5 => {
                        // Close Menu
                        self.despawn();
                    }
                    _ => unreachable!(),
                }
            }

            if pressed(ctx, Control::Up) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = self.buttons.len() - 1;
                }
            }
            if pressed(ctx, Control::Down) {
                if self.cursor < self.buttons.len() - 1 {
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                }
            }
        }
    }

    pub fn draw<'d>(&self, ctx: &mut EngineContext, dex: &PokedexClientContext<'d>, save: &PlayerData) {
        if self.alive {
            if self.bag.alive() {
                self.bag.draw(ctx, dex, &save.bag.items);
            } else if self.party.alive() {
                self.party.draw(ctx, &save.party);
            } else {
                Panel::draw_text(
                    ctx,
                    self.pos.x,
                    self.pos.y,
                    70.0,
                    &self.buttons,
                    self.cursor,
                    false,
                    false,
                );
            }
        }
    }

    pub fn fullscreen(&self) -> bool {
        self.party.alive() || self.bag.alive()
    }

    pub fn spawn_party<'d>(&mut self, ctx: &PokedexClientContext<'d>, save: &PlayerData<'d>) {
        self.spawn();
        spawn_party(ctx, &self.party, save)
    }
}

fn spawn_party<'d>(ctx: &PokedexClientContext<'d>, party: &PartyGui, save: &PlayerData<'d>) {
    party.spawn(ctx, &save.party, Some(true), true);
}

impl Entity for StartMenu {
    fn spawn(&mut self) {
        self.alive = true;
        self.cursor = 0;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
