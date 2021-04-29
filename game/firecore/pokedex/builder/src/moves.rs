use std::ffi::OsString;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use firecore_pokedex::moves::PokemonMove;
// use firecore_pokedex::serialize::SerializedMove;
// use firecore_pokedex::moves::battle_script::BattleActionScript;

pub fn get_moves<P: AsRef<std::path::Path>>(move_dir: P) -> Vec<PokemonMove> {
    let move_dir = move_dir.as_ref();
    read_dir(move_dir).unwrap_or_else(|err| panic!("Could not read moves directory at {:?} with error {}", move_dir, err))
        .map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => {
                Some(if path.is_dir() {
                    from_dir(path)
                } else {
                    // SerializedMove {
                        from_path(path)
                        // action_script: None,
                    // }
                })
            }
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            },
        }).flatten().collect()
}

fn from_dir(path: PathBuf) -> PokemonMove {
    for entry in read_dir(&path).unwrap_or_else(|err| panic!("Could not read move entry directory at {:?} with error {}", path, err)) {
        match entry.map(|entry| entry.path()) {
            Ok(path) => {
                if let Some(pokemon_move) = {
                    if let Some(extension) = path.extension() {
                        if extension == &OsString::from("ron") {
                            let data = read_to_string(&path).unwrap_or_else(|err| panic!("Could not read move at {:?} to string with error {}", path, err));
                            ron::from_str::<PokemonMove>(&data).ok()//.unwrap_or_else(|err| panic!("Could not deserialize move at {:?} with error {}", path, err)))
                        } else if extension == &OsString::from("toml") {
                            let data = read_to_string(&path).unwrap_or_else(|err| panic!("Could not read move at {:?} to string with error {}", path, err));
                            toml::from_str(&data).ok()//.unwrap_or_else(|err| panic!("Could not deserialize move at {:?} with error {}", path, err)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } {
                    // let action_script = read_to_string(path.parent().unwrap().join("actions.ron")).ok().map(|data| ron::from_str::<BattleActionScript>(&data).unwrap_or_else(|err| panic!("Could not parse actions script for move {} with error {}", pokemon_move.name, err)));
                    // return SerializedMove {
                    //     pokemon_move,
                    //     action_script,
                    // };
                    return pokemon_move;
                }                
            }
            Err(err) => {
                eprintln!("Could not read entry under move entry directory with error {}", err);
            }
        }
    }
    panic!("Could not get pokemon move entry under directory {:?}", path);
}

fn from_path(path: PathBuf) -> PokemonMove {
    let data = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("Could not read move at {:?} to string with error {}", path, err));
    if path.extension() == Some(&OsString::from("ron")) {
        ron::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize move at {:?} with error {}", path, err))
    } else {
        toml::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize move at {:?} with error {}", path, err))
    }
}