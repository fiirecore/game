use crate::{
    util::Entity,
    input::{pressed, Control},
    storage::player::SHOULD_SAVE,
    gui::{Panel, party::PartyGui, bag::BagGui},
    macroquad::prelude::Vec2,
    state::GameStateAction,
    quit,
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2,
    panel: Panel,
    buttons: [&'static str; 6],
    cursor: usize,
}

impl StartMenu {

    pub fn new() -> Self {
        Self {
            alive: false,
            pos: Vec2::new(169.0, 1.0),
            panel: Panel::new(),
            buttons: [
                "Save",
                "Bag",
                "Pokemon",
                "Menu",
                "Exit",
                "Cancel",
            ],
            cursor: 0,
        }
    }

    pub fn update(&mut self, delta: f32, party_gui: &mut PartyGui, bag_gui: &mut BagGui, action: &mut Option<GameStateAction>) {
        if bag_gui.alive {
            bag_gui.input();
            // bag_gui.up
        } else if party_gui.alive {
            party_gui.input();
            party_gui.update(delta);
        } else {
            
            if pressed(Control::B) || pressed(Control::Start) {
                self.despawn();
            }
    
            if pressed(Control::A) {
                match self.cursor {
                    0 => {
                        // Save
                        // #[deprecated(note = "change this")]
                        SHOULD_SAVE.store(true, std::sync::atomic::Ordering::Relaxed);
                    },
                    1 => {
                        // Bag
                        bag_gui.spawn();
                    },
                    2 => {
                        // Pokemon
                        party_gui.spawn_world();
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
    
            if pressed(Control::Up) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = self.buttons.len() - 1;
                }    
            }
            if pressed(Control::Down) {
                if self.cursor < self.buttons.len() - 1 {
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                }
            }
        }
    }

    pub fn render(&self) {
        if self.alive {
            self.panel.render_text(self.pos.x, self.pos.y, 70.0, &self.buttons, self.cursor, false, false);            
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