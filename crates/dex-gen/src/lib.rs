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

// pub fn generate() -> DexGenerator {
//     // std::env::set_var("SMOL_THREADS", &std::ffi::OsString::from("10"));

//     let start = std::time::Instant::now();

//     let client = client();

//     let pokerust2 = client.clone();

//     // let client_ = client.clone();

//     let moves_thread = std::thread::spawn(|| {
//         moves::add_moves(pokerust2)
//     });

//     let items_thread = std::thread::spawn(|| {
//         items::add_items(client)
//     });

//     let (moves, battle_moves) = moves_thread.join().unwrap(); //moves_thread.join().unwrap();

//     let (items, item_textures) = items_thread.join().unwrap();

//     let elapsed = start.elapsed().as_millis() as f64 / 1000.0;

//     println!("Finished in {} seconds!", elapsed);

//     DexGenerator {
//         moves: GeneratedMoves {
//             moves,
//             execution: battle_moves,
//         },
//         items: GeneratedItems {
//             items,
//             textures: item_textures,
//         }
//     }
// }

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
