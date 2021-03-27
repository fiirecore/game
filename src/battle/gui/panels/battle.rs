use firecore_util::text::TextColor;
use firecore_pokedex::pokemon::instance::PokemonInstance;
use firecore_util::Entity;
use firecore_input::{self as input, Control};
use macroquad::prelude::Vec2;
use macroquad::prelude::warn;
use crate::battle::Battle;
use crate::util::graphics::Texture;
use crate::gui::text::StaticText;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct BattlePanel {

    active: bool,

	pos: Vec2,
    panel: Vec2,
    
    background: Texture,

    pub fight_button: StaticText,
    pub bag_button: StaticText,
    pub pokemon_button: StaticText,
    pub run_button: StaticText,

    text: StaticText,
    
    cursor: usize,

    pub spawn_fight_panel: bool,

}

impl BattlePanel {

	pub fn new(panel: Vec2) -> Self {

        let pos = Vec2::new(121.0, 0.0);

        let font_id = 0;

        let text_x = 17.0;
        let text_y = 10.0;

		Self {

            active: false,
			
			pos,
            panel,
            
            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/button_panel.png")),

            fight_button: StaticText::new(vec!["FIGHT".to_owned()], TextColor::Black, font_id, false, Vec2::new(text_x, text_y), pos + panel),
            bag_button: StaticText::new(vec!["BAG".to_owned()], TextColor::Black, font_id, false, Vec2::new(text_x + 56.0, text_y), pos + panel),
            pokemon_button: StaticText::new(vec!["POKEMON".to_owned()], TextColor::Black, font_id,false,  Vec2::new(text_x, text_y + 16.0), pos + panel),
            run_button: StaticText::new(vec!["RUN".to_owned()], TextColor::Black, font_id, false, Vec2::new(text_x + 56.0, text_y + 16.0), pos + panel),

            text: StaticText::new(vec![String::from("What will"), String::from("POKEMON do?")], TextColor::White, 1, false, Vec2::new(-111.0, 10.0), pos + panel),

            cursor: 0,

            spawn_fight_panel: false,

		}
    }
    
    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.text.text[1] = instance.name() + " do?";
    }

    pub fn input(&mut self, _delta: f32, battle: &mut Battle) {
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
                    crate::gui::game::party::spawn();
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

            draw(self.background, self.panel.x + self.pos.x, self.panel.y + self.pos.y);

            self.text.render();
            self.fight_button.render();
            self.bag_button.render();
            self.pokemon_button.render();
            self.run_button.render();

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

            crate::util::graphics::draw_cursor(self.panel.x + self.pos.x + 10.0 + cursor_x, self.panel.y + self.pos.y + 13.0 + cursor_y);
		}		
	}

}

impl Entity for BattlePanel {

	fn spawn(&mut self) {
        self.active = true;
        self.cursor = 0;
        self.spawn_fight_panel = false;
        self.fight_button.spawn();
        self.bag_button.spawn();
        self.pokemon_button.spawn();
        self.run_button.spawn();
	}

	fn despawn(&mut self) {
        self.active = false;
        self.fight_button.despawn();
        self.bag_button.despawn();
        self.pokemon_button.despawn();
        self.run_button.despawn();
	}

	fn is_alive(& self) -> bool {
		self.active
	}

}