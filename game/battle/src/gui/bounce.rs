use super::pokemon::player::PlayerPokemonGui;

pub struct PlayerBounce {

    pub offset: f32,
    invert: bool,

}

const MAX: f32 = 1.0;
const MIN: f32 = 0.0;

impl PlayerBounce {

    pub fn new() -> Self {

        Self {
			
			invert: true,
            
            offset: MAX,

        }

    }

    pub fn reset(&mut self) {
        self.offset = MIN;
    }

    pub fn update(&mut self, delta: f32, player_pokemon_gui: &mut PlayerPokemonGui) {

        if self.invert {
            self.offset += 3.0 * delta;
            if self.offset >= MAX {
                self.offset = MAX;
                self.invert = false;
            }
        } else {
            self.offset -= 3.0 * delta;
            if self.offset <= MIN {
                self.offset = MIN;
                self.invert = true;
            }
        }
        player_pokemon_gui.vertical_offset(-self.offset);
    }

}