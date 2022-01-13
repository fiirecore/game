use std::rc::Rc;

use worldcli::worldlib::events::Sender;

use worldcli::worldlib::character::trainer::Trainer;

use crate::pokedex::pokemon::owned::SavedPokemon;

use crate::pokengine::{
    gui::{bag::BagGui, party::PartyGui},
    PokedexClientData,
};

use crate::state::game::GameActions;

use crate::engine::{
    controls::{pressed, Control},
    gui::Panel,
    log::info,
    math::Vec2,
    utils::Entity,
    Context, EngineContext,
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2,
    buttons: [&'static str; 5],
    cursor: usize,
    dex: Rc<PokedexClientData>,
    party: PartyGui,
    bag: BagGui,
    actions: Sender<GameActions>,
}

impl StartMenu {
    pub(crate) fn new(
        dex: Rc<PokedexClientData>,
        sender: Sender<GameActions>,
    ) -> Self {
        Self {
            alive: false,
            pos: Vec2::new(169.0, 1.0),
            buttons: ["Save", "Bag", "Pokemon", "Menu", "Cancel"],
            cursor: 0,
            party: PartyGui::new(&dex),
            bag: BagGui::new(&dex),
            dex,
            actions: sender,
        }
    }

    pub fn update(&mut self, ctx: &Context, eng: &mut EngineContext, delta: f32, user: &mut Trainer) {
        if self.bag.alive() {
            self.bag.input(ctx, eng, &mut user.bag);
            // bag_gui.up
        } else if self.party.alive() {
            self.party
                .input(ctx, eng, &self.dex, crate::dex::pokedex(), &mut user.party);
            self.party.update(delta);
        } else {
            if pressed(ctx, eng, Control::B) || pressed(ctx, eng, Control::Start) {
                self.despawn();
            }

            if pressed(ctx, eng, Control::A) {
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
                        self.spawn_party(&user.party);
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

            if pressed(ctx, eng, Control::Up) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = self.buttons.len() - 1;
                }
            }
            if pressed(ctx, eng, Control::Down) {
                if self.cursor < self.buttons.len() - 1 {
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        if self.alive {
            if self.bag.alive() {
                self.bag.draw(ctx, eng);
            } else if self.party.alive() {
                self.party.draw(ctx, eng);
            } else {
                Panel::draw_text(
                    ctx,
                    eng,
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

    pub fn spawn_party(&mut self, party: &[SavedPokemon]) {
        let pokedex = crate::dex::pokedex();
        if let Err(err) = self
            .party
            .spawn(&self.dex, pokedex, party, Some(true), true)
        {
            info!("Cannot spawn party GUI with error {}", err)
        }
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
