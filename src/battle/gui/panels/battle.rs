use firecore_util::text::TextColor;
use firecore_pokedex::pokemon::instance::PokemonInstance;
use firecore_util::Entity;
use firecore_input::{self as input, Control};
use macroquad::prelude::Texture2D;
use macroquad::prelude::warn;
use crate::battle::Battle;
use crate::gui::game::party::PokemonPartyGui;
use crate::util::graphics::{byte_texture, draw, draw_text_left, draw_cursor};
use crate::util::pokemon::PokemonTextures;

pub struct BattleOptions {

    active: bool,
    
    background: Texture2D,

    buttons: [String; 4],

    text: [String; 2],
    
    cursor: usize,

    pub spawn_fight_panel: bool,

}

impl BattleOptions {

	pub fn new() -> Self {

		Self {

            active: false,
            
            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/button_panel.png")),

            buttons: [
                "FIGHT".to_owned(),
                "BAG".to_owned(),
                "POKEMON".to_owned(),
                "RUN".to_owned()
            ],

            text: [
                "What will".to_owned(), 
                "POKEMON do?".to_owned(),
            ],

            cursor: 0,

            spawn_fight_panel: false,

		}
    }
    
    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.text[1] = instance.name() + " do?";
    }

    pub fn input(&mut self, battle: &mut Battle, party_gui: &mut PokemonPartyGui, textures: &PokemonTextures) {
        if input::pressed(Control::Up) {
            if self.cursor >= 2 {
                self.cursor -= 2;
            }            
        } else if input::pressed(Control::Down) {
            if self.cursor <= 2 {
                self.cursor += 2;
            } 
        } else if input::pressed(Control::Left) {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        } else if input::pressed(Control::Right) {
            if self.cursor < 3 {
                self.cursor += 1;
            }
        }

        if input::pressed(input::Control::A) {
            match self.cursor {
                0 => {
                    self.spawn_fight_panel = true;
                },
                1 => {
                    warn!("bag button unimplemented");
                },
                2 => {
                    party_gui.spawn_battle(textures, &battle.player);
                },
                3 => {
                    battle.run();
                },
                _ => {}
            }
        } 

    }

    pub fn render(&self) {
		if self.active {

            draw(self.background, 120.0, 113.0);

            for (index, string) in self.text.iter().enumerate() {
                draw_text_left(1, string, TextColor::White, 11.0, 123.0 + (index << 4) as f32);
            }

            for (index, string) in self.buttons.iter().enumerate() {
                draw_text_left(0, string, TextColor::Black, 138.0 + if index % 2 == 0 { 0.0 } else { 56.0 }, 123.0 + if index >> 1 == 0 { 0.0 } else { 16.0 })
            }

            let cursor_x = if self.cursor % 2 == 0 {
                0.0
            } else {
                56.0
            };
            let cursor_y = if (self.cursor >> 1) == 0 {
                0.0
            } else {
                16.0
            };

            draw_cursor(131.0 + cursor_x, 126.0 + cursor_y);
		}		
	}

}

impl Entity for BattleOptions {

	fn spawn(&mut self) {
        self.active = true;
        self.cursor = 0;
        self.spawn_fight_panel = false;
	}

	fn despawn(&mut self) {
        self.active = false;
	}

	fn is_alive(& self) -> bool {
		self.active
	}

}