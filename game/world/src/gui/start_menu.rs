use firecore_game::util::{Entity, text::TextColor};
use firecore_game::input::{pressed, Control};

use firecore_game::macroquad::prelude::{Vec2, Texture2D};

use firecore_game::gui::party::PokemonPartyGui;
use firecore_game::scene::{SceneState, Scenes};
use firecore_game::graphics::{byte_texture, draw, draw_text_left, draw_cursor};

pub struct StartMenu {

    alive: bool,

    pos: Vec2,

    background: Texture2D,

    buttons: Vec<String>,

    cursor: u8,

}

impl StartMenu {

    pub fn new() -> Self {

        Self {

            alive: false,

            pos: Vec2::new(169.0, 1.0),

            background: byte_texture(include_bytes!("../../assets/gui/world/start_menu.png")),

            buttons: vec![
                "Save",
                "Pokemon",
                "Main Menu",
                "Exit Game",
                "Close",
            ].into_iter().map(|text| text.to_ascii_uppercase()).collect(),

            cursor: 0,

        }

    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }

    pub fn input(&mut self, scene_state: &mut SceneState, party_gui: &mut PokemonPartyGui) {

        if pressed(Control::A) {
            match self.cursor {
                0 => {
                    // Save
                    firecore_game::data::DIRTY.store(true, std::sync::atomic::Ordering::Relaxed);
                },
                1 => {
                    // Pokemon
                    party_gui.spawn_world();
                },
                2 => {
                    // Exit to Main Menu
                    *scene_state = SceneState::Scene(Scenes::MainMenu);
                    self.despawn();
                },
                3 => {
                    // Exit Game
                    firecore_game::quit();
                },
                4 => {
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
                self.cursor = self.buttons.len() as u8 - 1;
            }    
        }
        if pressed(Control::Down) {
            if self.cursor < self.buttons.len() as u8 - 1 {
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