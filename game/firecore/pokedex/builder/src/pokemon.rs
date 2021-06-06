use std::fs::{read, read_to_string, read_dir};

use pokedex::pokemon::Pokemon;
use pokedex::serialize::SerializedPokemon;

pub fn get_pokemon<P: AsRef<std::path::Path>>(pokemon_dir: P, include_audio: bool) -> Vec<SerializedPokemon> {
    let pokemon_dir = pokemon_dir.as_ref();
    let mut pokemon = Vec::new();
    for entry in read_dir(pokemon_dir).unwrap_or_else(|err| panic!("Could not read pokemon directory at {:?} with error {}", pokemon_dir, err)) {
        match entry.map(|entry| entry.path()) {
            Ok(dir) => {
                if dir.is_dir() {

                    let file = dir.join("pokemon.ron");

                    let pokemon_entry = ron::from_str::<Pokemon>(
                        &read_to_string(&file)
                            .unwrap_or_else(|err| panic!("Could not read pokemon file at {:?} to string with error {}", file, err))
                    ).unwrap_or_else(|err| panic!("Could not parse pokemon file at {:?} with error {}", file, err));

                    let front_png = read(dir.join("normal_front.png"))
                        .unwrap_or_else(|err| panic!("Could not read front texture file for pokemon {} with error {}", pokemon_entry.name, err));

                    let back_png =  read(dir.join("normal_back.png"))
                        .unwrap_or_else(|err| panic!("Could not read back texture file for pokemon {} with error {}", pokemon_entry.name, err));

                    let icon_png = read(dir.join("icon.png"))
                        .unwrap_or_else(|err| panic!("Could not read icon file for pokemon {} with error {}", pokemon_entry.name, err));

                    let cry_ogg = {
                        if include_audio {
                            read(dir.join("cry.ogg")).ok().unwrap_or_default()
                        } else {
                            Vec::new()
                        }
                    };
        
                    pokemon.push(SerializedPokemon {
                        pokemon: pokemon_entry,
                        cry_ogg,
                        front_png,
                        back_png,
                        icon_png,
                    });
                }
            }
            Err(err) => eprintln!("Could not read directory entry with error {}", err),
        }
    }

    println!("Loaded {} pokemon.", pokemon.len());

    pokemon
}

// fn find_entry_file(dir_path: &Path) -> Option<Pokemon> {
//     for file_entry in read_dir(&dir_path).unwrap_or_else(|err| panic!("Could not read pokemon directory at {:?} with error {}", dir_path, err)) {
//         let file = file_entry.unwrap_or_else(|err| panic!("Could not get pokemon directory entry path under {:?} with error {}", dir_path, err)).path();
//         if let Some(ext) = file.extension() {
//             if ext == std::ffi::OsString::from("ext") {
//                 let data = std::fs::read_to_string(&file).unwrap_or_else(|err| panic!("Could not read pokemon file at {:?} to string with error {}", file, err));
//                 return Some(ron::from_str(&data).unwrap_or_else(|err| panic!("Could not parse pokemon file at {:?} with error {}", file, err)));
//             }
//         }
//     }
//     None
// }