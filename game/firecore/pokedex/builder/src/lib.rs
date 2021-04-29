use std::fs::File;
use std::io::Write;

pub mod pokemon;
pub mod moves;
pub mod items;

pub fn compile<P: AsRef<std::path::Path>>(pokemon_dir: P, move_dir: P, item_dir: P, output_file: P, include_audio: bool) {
    let output_file = output_file.as_ref();

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon_dir, include_audio);
    println!("Loading moves...");
    let moves = moves::get_moves(move_dir);
    println!("Loading items...");
    let items = items::get_items(item_dir);
    
    println!("Saving to file...");
    let size = File::create(output_file)
        .unwrap_or_else(|err| panic!("Could not create output file at {:?} with error {}", output_file, err))
            .write(
            &postcard::to_allocvec(
                    &firecore_pokedex::serialize::SerializedDex {
                        pokemon,
                        moves,
                        items,
                    }
                ).unwrap_or_else(|err| panic!("Could not serialize data with error {}", err))
    ).unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
    println!("Saved {} bytes to output file!", size);
}