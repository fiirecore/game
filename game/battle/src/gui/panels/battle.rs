use game::{
    util::{Entity, text::TextColor},
    pokedex::pokemon::instance::PokemonInstance,
    input::{self as input, Control},
    macroquad::prelude::{Texture2D, warn},
    gui::party::PokemonPartyGui,
    graphics::{byte_texture, draw, draw_text_left, draw_cursor},
};

use crate::Battle;
use crate::gui::pokemon::party::battle_party_gui;

pub struct BattleOptions {

    active: bool,
    
    background: Texture2D,

    buttons: [&'static str; 4],

    what_will: &'static str,
    pokemon_do: String,
    
    cursor: usize,

    pub spawn_fight_panel: bool,

}

impl BattleOptions {

	pub fn new() -> Self {

		Self {

            active: false,
            
            background: byte_texture(include_bytes!("../../../assets/gui/button_panel.png")),

            buttons: [
                "FIGHT",
                "BAG",
                "POKEMON",
                "RUN"
            ],

            what_will: "What will",
            pokemon_do: "POKEMON do?".to_owned(),

            cursor: 0,

            spawn_fight_panel: false,

		}
    }
    
    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.pokemon_do = instance.name() + " do?";
    }

    pub fn input(&mut self, battle: &mut Battle, party_gui: &mut PokemonPartyGui) {
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
                    battle_party_gui(party_gui, &battle.player);
                },
                3 => {
                    battle.try_run();
                },
                _ => {}
            }
        } 

    }

    pub fn render(&self) {
		if self.active {

            draw(self.background, 120.0, 113.0);

            draw_text_left(1, self.what_will, TextColor::White, 11.0, 123.0);
            draw_text_left(1, &self.pokemon_do, TextColor::White, 11.0, 139.0);

            for (index, string) in self.buttons.iter().enumerate() {
                draw_text_left(0, string, TextColor::Black, 138.0 + if index % 2 == 0 { 0.0 } else { 56.0 }, 123.0 + if index >> 1 == 0 { 0.0 } else { 16.0 })
            }

            draw_cursor(131.0 + if self.cursor % 2 == 0 {
                0.0
            } else {
                56.0
            }, 126.0 + if (self.cursor >> 1) == 0 {
                0.0
            } else {
                16.0
            });
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