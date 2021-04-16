use game::pokedex::moves::PokemonMove;

pub struct BattleMoveStatus {

    pub pokemon_move: PokemonMove,
    pub queued: bool,

}

impl BattleMoveStatus {

    pub fn new(pokemon_move: Option<&PokemonMove>) -> Option<Self> {
        pokemon_move.map(|pokemon_move| {
            Self {
                pokemon_move: pokemon_move.clone(),
                queued: true,
            }
        })        
    }

}