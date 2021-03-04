use crate::io::data::text::color::TextColor;
use firecore_pokedex::pokemon::battle::BattlePokemon;
use crate::util::Entity;
use crate::gui::Focus;
use crate::util::Input;
use frc_input::{self as input, Control};
use crate::gui::button::BasicButton;
use crate::util::Direction;
use crate::util::graphics::Texture;
use crate::gui::GuiComponent;
use crate::gui::text::StaticText;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct BattlePanel {

    active: bool,
    // focus: bool,

	x: f32,
    y: f32,
    panel_x: f32,
    panel_y: f32,
    
    background: Texture,
    
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub next: u8,

    pub fight_button: BasicButton,
    pub bag_button: BasicButton,
    pub pokemon_button: BasicButton,
    pub run_button: BasicButton,

    text: StaticText,

}

impl BattlePanel {

	pub fn new(panel_x: f32, panel_y: f32) -> Self {

        let x = 121.0;
        let y = 0.0;

        let font_id = 2;

        let text_y = 10.0;

		Self {

            active: false,
            // focus: false,
			
			x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,
            
            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/button_panel.png")),

            cursor_x: 0,
            cursor_y: 0,
            next: 0,

            fight_button: BasicButton::new("Fight".to_ascii_uppercase().as_str(), font_id, 17.0, text_y, x + panel_x, y + panel_y),
            bag_button: BasicButton::new("Bag".to_ascii_uppercase().as_str(), font_id, 73.0, text_y, x + panel_x, y + panel_y),
            pokemon_button: BasicButton::new("Pokemon".to_ascii_uppercase().as_str(), font_id, 17.0, text_y + 16.0, x + panel_x, y + panel_y),
            run_button: BasicButton::new("Run".to_ascii_uppercase().as_str(), font_id, 73.0, text_y + 16.0, x + panel_x, y + panel_y),

            text: StaticText::new(vec![String::from("What will"), String::from("POKEMON do?")], TextColor::White, 1, Direction::Left, -111.0, 10.0, x + panel_x, y + panel_y),

		}
    }
    
    pub fn update_text(&mut self, instance: &BattlePokemon) {
        self.text.text[1] = instance.pokemon.data.name.to_uppercase() + " do?";
    }

}

impl GuiComponent for BattlePanel {

	fn update(&mut self, _delta: f32) {}

	fn render(&self) {
		if self.is_alive() {
            draw(self.background, self.panel_x + self.x, self.panel_y + self.y);
            let cursor_x = if self.cursor_x > 0 {
                56.0
            } else {
                0.0
            };
            let cursor_y = if self.cursor_y > 0 {
                16.0
            } else {
                0.0
            };
            self.text.render();
            crate::util::graphics::draw_cursor(self.panel_x + self.x + 10.0 + cursor_x, self.panel_y + self.y + 13.0 + cursor_y);
            self.fight_button.render();
            self.bag_button.render();
            self.pokemon_button.render();
            self.run_button.render();
            
		}		
	}

	fn update_position(&mut self, panel_x: f32, panel_y: f32) {
        self.panel_x = panel_x as f32;
        self.panel_y = panel_y as f32;
        self.fight_button.update_position(panel_x, panel_y);
        self.bag_button.update_position(panel_x, panel_y);
        self.pokemon_button.update_position(panel_x, panel_y);
        self.run_button.update_position(panel_x, panel_y);
	}
}

impl Input for BattlePanel {

    fn input(&mut self, _delta: f32) {
        if self.is_alive() {
            if input::pressed(Control::Up) {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }            
            } else if input::pressed(Control::Down) {
                if self.cursor_y < 1 {
                    self.cursor_y += 1;
                } 
            } else if input::pressed(Control::Left) {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            } else if input::pressed(Control::Right) {
                if self.cursor_x < 1 {
                    self.cursor_x += 1;
                }
            }
        }        
    }

}

impl Entity for BattlePanel {

	fn spawn(&mut self) {
        self.active = true;
        self.fight_button.spawn();
        self.bag_button.spawn();
        self.pokemon_button.spawn();
        self.run_button.spawn();
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.next = 0;
	}

	fn despawn(&mut self) {
        self.active = false;
        // self.unfocus();
        self.fight_button.despawn();
        self.bag_button.despawn();
        self.pokemon_button.despawn();
        self.run_button.despawn();
        self.next = 0;
	}

	fn is_alive(& self) -> bool {
		self.active
	}

}

// impl Focus for BattlePanel {

//     fn focus(&mut self) {
//         self.focus = true;
//     }

//     fn unfocus(&mut self) {
//         self.focus = false;
//     }

//     fn in_focus(&mut self) -> bool {
//         self.focus
//     }

// }