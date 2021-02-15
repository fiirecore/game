
use crate::battle::battle_pokemon::BattlePokemon;
use crate::gui::Focus;
use crate::util::Input;
use crate::util::input::Control;
use crate::gui::button::BasicButton;
use crate::io::data::Direction;
use crate::util::graphics::Texture;
use crate::gui::GuiComponent;
use crate::util::input;
use crate::gui::text::StaticText;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct BattlePanel {

    active: bool,
    focus: bool,

	x: f32,
    y: f32,
    panel_x: f32,
    panel_y: f32,
    
    background: Texture,
    
    pub cursor_position: u8,
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
            focus: false,
			
			x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,
            
            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/button_panel.png")),

            cursor_position: 0,
            next: 0,

            fight_button: BasicButton::new("Fight".to_uppercase().as_str(), font_id, 17.0, text_y, x + panel_x, y + panel_y),
            bag_button: BasicButton::new("Bag".to_uppercase().as_str(), font_id, 73.0, text_y, x + panel_x, y + panel_y),
            pokemon_button: BasicButton::new("Pokemon".to_uppercase().as_str(), font_id, 17.0, text_y + 16.0, x + panel_x, y + panel_y),
            run_button: BasicButton::new("Run".to_uppercase().as_str(), font_id, 73.0, text_y + 16.0, x + panel_x, y + panel_y),

            text: StaticText::new(vec![String::from("What will"), String::from("POKEMON do?")], 1, Direction::Left, -111.0, 10.0, x + panel_x, y + panel_y),

		}
    }
    
    pub fn update_text(&mut self, instance: &BattlePokemon) {
        self.text.text[1] = instance.data.name.to_uppercase() + " do?";
    }

}

impl GuiComponent for BattlePanel {

	fn enable(&mut self) {
        self.active = true;
        self.fight_button.enable();
        self.bag_button.enable();
        self.pokemon_button.enable();
        self.run_button.enable();
        self.cursor_position = 0;
        self.next = 0;
	}

	fn disable(&mut self) {
        self.active = false;
        self.unfocus();
        self.fight_button.disable();
        self.bag_button.disable();
        self.pokemon_button.disable();
        self.run_button.disable();
        self.next = 0;
	}

	fn is_active(& self) -> bool {
		self.active
	}

	fn update(&mut self, _delta: f32) {}

	fn render(&self) {
		if self.is_active() {
            draw(self.background, self.panel_x + self.x, self.panel_y + self.y);
            let mut cursor_x = 0.0;
            let mut cursor_y = 0.0;
            match self.cursor_position {
                1 => {
                    cursor_x = 56.0;
                },
                2 => {
                    cursor_y = 16.0;
                },
                3 => {
                    cursor_x = 56.0;
                    cursor_y = 16.0;
                },
                _ => {}
            }
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

	fn load(&mut self) {

	}
}

impl Focus for BattlePanel {

    fn focus(&mut self) {
        self.focus = true;
    }

    fn unfocus(&mut self) {
        self.focus = false;
    }

    fn in_focus(&mut self) -> bool {
        self.focus
    }

    // fn next(&self) -> u8 {
    //     self.next
    // }

}

impl Input for BattlePanel {

    fn input(&mut self, _delta: f32) {
        if input::pressed(Control::Up) {
            if self.cursor_position >= 2 {
                self.cursor_position -= 2;
            }            
        } else if input::pressed(Control::Down) {
            if self.cursor_position < 2 {
                self.cursor_position += 2;
            } 
        } else if input::pressed(Control::Left) {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }
        } else if input::pressed(Control::Right) {
            if self.cursor_position < 3 {
                self.cursor_position += 1;
            }
        }
    }

}