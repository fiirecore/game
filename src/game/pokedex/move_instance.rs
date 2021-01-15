use crate::io::data::pokemon::moves::pokemon_move::PokemonMove;

pub struct MoveInstance {
	
	pub move_instance: PokemonMove,
	pub remaining_pp: u8,
	
}

impl MoveInstance {

	pub fn use_move(&mut self) -> PokemonMove {
		self.remaining_pp -= 1;
		self.move_instance.clone()
	}

}