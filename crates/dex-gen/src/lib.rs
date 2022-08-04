extern crate firecore_battle as battle;
// #![feature(exclusive_range_pattern)]

use std::sync::Arc;

use battle::pokedex::types::PokemonType;

pub mod items;
pub mod moves;
pub mod pokemon;

// pub(crate) const EXTENSION: &str = "ron";

pub type Client = Arc<pokerust::Client>;

pub fn client() -> Client {
    Arc::new(pokerust::Client::default())
}

#[inline]
pub(crate) fn capitalize_first(string: &mut String) {
    string[..1].make_ascii_uppercase();
}

pub(crate) fn type_from_id(id: i16) -> PokemonType {
    match id {
        1 => PokemonType::Normal,
        10 => PokemonType::Fire,
        11 => PokemonType::Water,
        13 => PokemonType::Electric,
        12 => PokemonType::Grass,
        15 => PokemonType::Ice,
        2 => PokemonType::Fighting,
        4 => PokemonType::Poison,
        5 => PokemonType::Ground,
        3 => PokemonType::Flying,
        14 => PokemonType::Psychic,
        7 => PokemonType::Bug,
        6 => PokemonType::Rock,
        8 => PokemonType::Ghost,
        16 => PokemonType::Dragon,
        17 => PokemonType::Dark,
        9 => PokemonType::Steel,
        18 => PokemonType::Fairy,
        _ => panic!("Could not get pokemon type from id \"{}\"", id),
    }
}
