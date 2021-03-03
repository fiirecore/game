use serde::{Deserialize, Serialize};

use super::instance::{MoveInstance, MoveInstances};

pub type SerializableMoveSet = Vec<SerializablePokemonMove>;

pub fn to_instances(moves: &SerializableMoveSet) -> MoveInstances {
    let mut move_instances = Vec::new();
    for saved_move in moves {
        if let Some(pokemon_move) = crate::MOVEDEX.get(&saved_move.move_id) {
            move_instances.push(MoveInstance {
                pokemon_move: pokemon_move,
                remaining_pp: saved_move.remaining_pp,
            });
        } else {
            // macroquad::prelude::warn!("Could not get pokemon move from id {}!", saved_move.move_id);
        }
    }
    return move_instances;
}

pub fn from_instances(moves: &MoveInstances) -> SerializableMoveSet {
    moves.iter().map(|move_instance| SerializablePokemonMove {
        move_id: move_instance.pokemon_move.number,
        remaining_pp: move_instance.remaining_pp,
    }).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePokemonMove {

	pub move_id: u16,
	pub remaining_pp: u8,
	
}