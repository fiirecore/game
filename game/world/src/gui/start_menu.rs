use game::{
    util::{Entity, text::TextColor},
    input::{pressed, Control},
    macroquad::prelude::{Vec2, Texture2D},
    gui::{party::PartyGui, bag::BagGui},
    graphics::{byte_texture, draw, draw_text_left, draw_cursor},
    state::GameStateAction,
};

pub struct StartMenu {

    alive: bool,

    pos: Vec2,

    background: Texture2D,

    buttons: [&'static str; 6],

    cursor: usize,

}

impl StartMenu {

    pub fn new() -> Self {

        Self {

            alive: false,

            pos: Vec2::new(169.0, 1.0),

            background: byte_texture(include_bytes!("../../assets/gui/world/start_menu.png")),

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

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }

    pub fn input(&mut self, action: &mut Option<GameStateAction>, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {

        if pressed(Control::A) {
            match self.cursor {
                0 => {
                    // Save
                    #[deprecated(note = "change to function")]
                    firecore_game::data::DIRTY.store(true, std::sync::atomic::Ordering::Relaxed);
                },
                1 => {
                    // Bag
                    bag_gui.spawn(true);
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
                    firecore_game::quit();
                },
                5 => {
                    // Close Menu
                    self.despawn();
                }
                _ => (),
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

    pub fn render(&self) {
        if self.alive {
            draw(self.background, self.pos.x, self.pos.y);
            for (index, text) in self.buttons.iter().enumerate() {
                draw_text_left(1, text, TextColor::Black, self.pos.x + 15.0, self.pos.y + 7.0 + (index << 4) as f32);
            }
            draw_cursor(self.pos.x + 8.0, self.pos.y + 9.0 + (self.cursor << 4) as f32);
        }
    }

}

impl Entity for StartMenu {

    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

}