use serde::{Deserialize, Serialize};
use deps::hash::HashMap;
use pokedex::item::ItemId;
use util::Coordinate;

#[derive(Serialize, Deserialize)]
pub struct Pokemart {

    #[serde(skip)]
    pub alive: bool,
    pub inventory: HashMap<ItemId, u16>,

}

impl Pokemart {

    const POS: Coordinate = Coordinate { x: 4, y: 3 };

    pub fn activate(&mut self, coords: Coordinate) {
        if coords == Self::POS {
            self.alive = true;
        }
    }

}