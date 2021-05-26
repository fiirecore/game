use crate::{
    util::Entity,
    input::{pressed, Control},
    storage::player::SHOULD_SAVE,
    gui::{Panel, party::PartyGui, bag::BagGui},
    tetra::{Context, math::Vec2},
    state::GameStateAction,
    quit,
};

pub struct StartMenu {
    alive: bool,
    pos: Vec2<f32>,
    panel: Panel,
    buttons: [&'static str; 6],
    cursor: usize,
}

impl StartMenu {

    pub fn new(ctx: &mut Context) -> Self {
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
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32, party_gui: &mut PartyGui, bag_gui: &mut BagGui, action: &mut Option<GameStateAction>) {
        if bag_gui.alive {
            bag_gui.input(ctx);
            // bag_gui.up
        } else if party_gui.alive {
            party_gui.input(ctx);
            party_gui.update(delta);
        } else {
            
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
            self.panel.draw_text(ctx, self.pos.x, self.pos.y, 70.0, &self.buttons, self.cursor, false, false);            
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