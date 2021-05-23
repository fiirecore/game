use firecore_pokedex_game::serialize::SerializedDex;
use firecore_pokedex_game::{
    pokemon::POKEDEX,
    moves::MOVEDEX
};

pub(crate) fn setup(dex: SerializedDex) {
    // let dex = firecore_dependencies::ser::deserialize::<firecore_pokedex_game::serialize::SerializedDex>(&std::fs::read(dex_bin).unwrap()).unwrap();
    unsafe { POKEDEX = Some(dex.pokemon.into_iter().map(|ser_pkmn| (ser_pkmn.pokemon.id, ser_pkmn.pokemon)).collect()) }
    unsafe { MOVEDEX = Some(dex.moves.into_iter().map(|ser_move| (ser_move.pokemon_move.id, ser_move.pokemon_move)).collect()) }
}