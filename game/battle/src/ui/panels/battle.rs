use game::{
    pokedex::pokemon::instance::PokemonInstance,
    input::{self as input, Control},
    gui::Panel,
    text::TextColor,
    graphics::{draw_text_left, draw_cursor},
};

pub struct BattleOptions {
    
    panel: Panel,

    buttons: [&'static str; 4],

    pokemon_do: String,
    
    pub cursor: usize,

}

impl BattleOptions {

	pub fn new() -> Self {

		Self {
            
            panel: Panel::new(),

            buttons: [
                "FIGHT",
                "BAG",
                "POKEMON",
                "RUN"
            ],

            pokemon_do: "POKEMON do?".to_owned(),

            cursor: 0,

		}
    }
    
    pub fn setup(&mut self, instance: &PokemonInstance) {
        self.pokemon_do = format!("{} do?", instance.name());
    }

    pub fn input(&mut self) {
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
    }

    pub fn render(&self) {
        self.panel.render(120.0, 113.0, 120.0, 47.0);

        draw_text_left(1, "What will", TextColor::White, 11.0, 123.0);
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