use dashmap::mapref::one::Ref;

use crate::MoveId;

use super::PokemonMove;

pub type MoveInstances = Vec<MoveInstance>;

pub struct MoveInstance {
	
	pub pokemon_move: Ref<'static, MoveId, PokemonMove>,
	pub remaining_pp: u8,
	
}

impl MoveInstance {

	pub fn use_move(&mut self) -> &PokemonMove {
		self.remaining_pp -= 1;
		&self.pokemon_move
	}

}