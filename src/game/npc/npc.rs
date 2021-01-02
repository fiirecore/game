use crate::entity::util::direction::Direction;
use crate::io::data::player_data::SavedPokemon;

pub struct NPC {

    // x and y local to map

    pub x: isize,
    pub y: isize,

    pub direction: Direction,
    pub sprite: u8,

    pub pokemon: Option<Vec<SavedPokemon>>,

}

impl NPC {

    pub fn interact(&mut self, direction: Direction) {
        self.direction = direction.inverse();
    }

}