use firecore_world::events::Sender;

use crate::{
    saves::{GameBag, GameParty},
};

use std::rc::Rc;

use crate::engine::{
    gui::Panel,
    input::controls::{pressed, Control},
    math::Vec2,
    utils::Entity,
    Context,
};
use crate::saves::PlayerData;
use crate::{
    pokedex::{
        PokedexClientData,
        gui::{bag::BagGui, party::PartyGui},
    },
    state::game::GameActions,
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2,
    buttons: [&'static str; 5],
    cursor: usize,
    dex: Rc<PokedexClientData>,
    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
    actions: Sender<GameActions>,
}

impl StartMenu {
    pub(crate) fn new(
        dex: Rc<PokedexClientData>,
        party: Rc<PartyGui>,
        bag: Rc<BagGui>,
        sender: Sender<GameActions>,
    ) -> Self {
        Self {
            alive: false,
            pos: Vec2::new(169.0, 1.0),
            buttons: ["Save", "Bag", "Pokemon", "Menu", "Cancel"],
            cursor: 0,
            dex,
            party,
            bag,
            actions: sender,
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32, party: &mut GameParty, bag: &GameBag) {
        if self.bag.alive() {
            self.bag.input(ctx, bag);
            // bag_gui.up
        } else if self.party.alive() {
            self.party.input(ctx, &self.dex, party);
            self.party.update(delta);
        } else {
            if pressed(ctx, Control::B) || pressed(ctx, Control::Start) {
                self.despawn();
            }

            if pressed(ctx, Control::A) {
                match self.cursor {
                    0 => {
                        // Save
                        self.actions.send(GameActions::Save);
                    }
                    1 => {
                        // Bag
                        self.bag.spawn();
                    }
                    2 => {
                        // Pokemon
                        self.spawn_party(party);
                    }
                    3 => {
                        // Exit to Main Menu
                        self.actions.send(GameActions::Exit);
                        self.despawn();
                    }
                    4 => {
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

    pub fn draw<'d>(&self, ctx: &mut Context, save: &PlayerData) {
        if self.alive {
            if self.bag.alive() {
                self.bag.draw(ctx, &self.dex, &save.bag);
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

    pub fn spawn_party(&mut self, party: &GameParty) {
        self.party.spawn(&self.dex, party, Some(true), true);
    }
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
