use crate::game::pokedex::pokemon_move::pokemon_move::PokemonMove;

pub struct UsedMove {
	
	pub attacker_atk: usize,
	pub attacker_sp_atk: usize,
	pub _move: PokemonMove,
	
}

impl UsedMove {
	
	pub fn new(_attacker_atk: usize, _attacker_sp_atk: usize, __move: PokemonMove) -> UsedMove {
		
		UsedMove {
			
			_move: __move,
			attacker_atk: _attacker_atk,
			attacker_sp_atk: _attacker_sp_atk,
			
		}
		
	}
	
}