extern crate firecore_pokedex_game as pokedex;

use std::fs::File;
use std::io::Write;

use pokedex::serialize::SerializedDex;

pub mod pokemon;
pub mod moves;
pub mod items;
pub mod trainers;

// #[cfg(feature = "gen")]
pub mod gen;

pub fn compile<P: AsRef<std::path::Path>>(pokemon_dir: P, move_dir: P, item_dir: P, trainer_dir: P, output_file: P, include_audio: bool, save: bool) -> SerializedDex {
    let output_file = output_file.as_ref();

    // #[cfg(feature = "gen")]
    // gen::gen(pokemon_dir, move_dir)

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon_dir, include_audio);
    println!("Loading moves...");
    let moves = moves::get_moves(move_dir);
    println!("Loading items...");
    let items = items::get_items(item_dir);
    println!("Loading trainer textures...");
    let trainers = trainers::get_trainers(trainer_dir);
    
    let dex = SerializedDex {
        pokemon,
        moves,
        items,
        trainers,
    };

    if save {
        println!("Saving to file...");
        let size = File::create(output_file)
            .unwrap_or_else(|err| panic!("Could not create output file at {:?} with error {}", output_file, err))
                .write(
                &firecore_dependencies::ser::serialize(
                        &dex
                    ).unwrap_or_else(|err| panic!("Could not serialize data with error {}", err))
        ).unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
        println!("Saved {} bytes to output file!", size);
    }

    dex
}