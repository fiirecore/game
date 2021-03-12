use super::pokemon_gui::PlayerPokemonGui;
use super::pokemon_gui::PokemonGui;

pub struct PlayerBounce {

    counter: f32,
	pokemon_up: bool,
    gui_up: bool,
    
    pokemon_offset: u8,

}

static ONE_THIRD: f32 = 1.0/3.0;

impl PlayerBounce {

    pub fn new() -> Self {

        Self {

            counter: 0.0,
			
			pokemon_up: false,
            gui_up: true,
            
            pokemon_offset: 1,

        }

    }

    pub fn reset(&mut self) {
        self.pokemon_up = false;
        self.gui_up = true;
        self.pokemon_offset = 1;
    }

    pub fn update(&mut self, delta: f32, player_pokemon_gui: &mut PlayerPokemonGui) {
        self.counter = (self.counter + delta) % ONE_THIRD;
        if self.counter < delta {
            self.pokemon_up = !self.pokemon_up;
            self.pokemon_offset = if self.pokemon_up {
                1
            } else {
                0
            };
        }
        let panel = self.counter - ONE_THIRD / 2.0;
        if panel >= 0.0 && panel < delta {
            self.gui_up = !self.gui_up;
            if self.gui_up {
                player_pokemon_gui.offset_position(0.0, -1.0);
            } else {
                player_pokemon_gui.offset_position(0.0, 1.0);
            };
        }
    }

    pub fn pokemon_offset(&self) -> u8 {
        self.pokemon_offset
    }

}