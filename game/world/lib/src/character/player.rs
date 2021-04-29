// use util::Coordinate;

use super::Character;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct PlayerCharacter {

	// pub global: Coordinate,
	pub character: Character,

	pub input_frozen: bool,

}

impl PlayerCharacter {

    pub fn do_move(&mut self, delta: f32) -> bool {
        if !self.character.position.offset.is_none() {
            if self.character.position.offset.update(delta * self.character.speed, &self.character.position.direction) {
                self.character.position.coords += self.character.position.direction.tile_offset();
                self.character.update_sprite();
                true
            } else {
                false
            }
        } else {
            false
        }        
    }

    pub fn is_frozen(&self) -> bool {
        self.input_frozen || self.character.is_frozen()
    }

    pub fn freeze_input(&mut self) {
        self.input_frozen = true;
        self.character.stop_move();
    }

    pub fn unfreeze(&mut self) {
        self.input_frozen = false;
        self.character.unfreeze();
    }

}