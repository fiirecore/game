use super::pokemon_gui::PlayerPokemonGui;
use super::pokemon_gui::PokemonGui;

pub struct PlayerBounce {

    counter: u8,
	pokemon_up: bool,
    gui_up: bool,
    
    pokemon_offset: u8,

}

impl PlayerBounce {

    pub fn new() -> Self {

        Self {

            counter: 0,
			
			pokemon_up: false,
            gui_up: true,
            
            pokemon_offset: 1,

        }

    }

    pub fn update(&mut self, player_pokemon_gui: &mut PlayerPokemonGui) {
        self.counter = (self.counter + 1) % 20;
        if self.counter == 0 {
            self.pokemon_up = !self.pokemon_up;
            self.pokemon_offset = if self.pokemon_up {
                1
            } else {
                0
            };
        }
        if self.counter == 10 {
            self.gui_up = !self.gui_up;
            if self.gui_up {
                player_pokemon_gui.offset_position(0, -1);
            } else {
                player_pokemon_gui.offset_position(0, 1);
            };
        }
    }

    pub fn pokemon_offset(&self) -> u8 {
        self.pokemon_offset
    }

}