use firecore_pokedex_game::{
    serialize::SerializedDex,
    pokemon::dex::set as mon,
    moves::dex::set as mov,
};

pub(crate) fn setup(dex: SerializedDex) {
    mon(dex.pokemon.into_iter().map(|ser_pkmn| (ser_pkmn.pokemon.id, ser_pkmn.pokemon)).collect());
    mov(dex.moves.into_iter().map(|ser_move| (ser_move.pokemon_move.id, ser_move.pokemon_move)).collect());
}