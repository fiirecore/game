use super::PokemonMove;

pub type MoveInstances = Vec<MoveInstance>;

#[derive(Debug)]
pub struct MoveInstance {
	
	pub pokemon_move: PokemonMove,
	pub remaining_pp: u8,
	
}

impl MoveInstance {

	pub fn use_move(&mut self) -> &PokemonMove {
		self.remaining_pp -= 1;
		&self.pokemon_move
	}

}