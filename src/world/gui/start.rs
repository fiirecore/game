use std::{borrow::Cow, rc::Rc};

use crate::{
    util::Entity,
    input::{pressed, Control},
    storage::{data, data_mut, player::SHOULD_SAVE},
    gui::{Panel, party::PartyGui, bag::BagGui, pokemon::PokemonDisplay},
    tetra::{Context, math::Vec2},
    game::GameStateAction,
    quit,
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2<f32>,
    panel: Panel,
    buttons: [&'static str; 6],
    cursor: usize,
    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
}

impl StartMenu {

    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            alive: false,
            pos: Vec2::new(169.0, 1.0),
            panel: Panel::new(ctx),
            buttons: [
                "Save",
                "Bag",
                "Pokemon",
                "Menu",
                "Exit",
                "Cancel",
            ],
            cursor: 0,
            party,
            bag,
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32, input_lock: bool, action: &mut Option<GameStateAction>) {
        if self.bag.alive() && !input_lock {
            self.bag.input(ctx);
            // bag_gui.up
        } else if self.party.alive() {
            if !input_lock {
                self.party.input(ctx, data_mut().party.as_mut_slice());
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
                        // #[deprecated(note = "change this")]
                        SHOULD_SAVE.store(true, std::sync::atomic::Ordering::Relaxed);
                    },
                    1 => {
                        // Bag
                        self.bag.spawn(&mut data_mut().bag);
                    },
                    2 => {
                        // Pokemon
                        spawn_party(&self.party);
                    },
                    3 => {
                        // Exit to Main Menu
                        *action = Some(GameStateAction::ExitToMenu);
                        self.despawn();
                    },
                    4 => {
                        // Exit Game
                        quit();
                    },
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

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            if self.bag.alive() {
                self.bag.draw(ctx);
            } else if self.party.alive() {
                self.party.draw(ctx);
            } else {
                self.panel.draw_text(ctx, self.pos.x, self.pos.y, 70.0, &self.buttons, self.cursor, false, false);    
            }        
        }
    }

    pub fn fullscreen(&self) -> bool {
        self.party.alive() || self.bag.alive()
    }

    pub fn spawn_party(&mut self) {
        self.spawn();
        spawn_party(&self.party)
    }

}

fn spawn_party(party: &PartyGui) {
    party.spawn(data().party.iter().map(|instance| PokemonDisplay::new(Cow::Borrowed(instance))).collect(), Some(true), true);
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