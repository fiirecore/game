use firecore_pokedex_game::{
    serialize::SerializedDex,
    Dex,
    pokemon::Pokedex,
    moves::Movedex,
};

pub(crate) fn setup(dex: SerializedDex) {
    Pokedex::set(dex.pokemon.into_iter().map(|ser_pkmn| (ser_pkmn.pokemon.id, ser_pkmn.pokemon)).collect());
    Movedex::set(dex.moves.into_iter().map(|ser_move| (ser_move.pokemon_move.id, ser_move.pokemon_move)).collect());
}