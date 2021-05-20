use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use pokedex::moves::{Move, GamePokemonMove};
use pokedex::serialize::SerializedMove;

pub fn get_moves<P: AsRef<std::path::Path>>(move_dir: P) -> Vec<SerializedMove> {
    let move_dir = move_dir.as_ref();
    read_dir(move_dir).unwrap_or_else(|err| panic!("Could not read moves directory at {:?} with error {}", move_dir, err))
        .map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => {
                Some(if path.is_dir() {
                    from_dir(path)
                } else {
                    SerializedMove::from(from_file(path))
                })
            }
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            },
        }).flatten().collect()
}

fn from_dir(dir: PathBuf) -> SerializedMove {
    let toml_path = dir.join("move.toml");
    let ron_path = dir.join("move.ron");
    if !toml_path.exists() {
        ron::from_str(
            &read_to_string(&ron_path)
                .unwrap_or_else(|err| panic!("Could not read move file at {:?} to string with error {}", ron_path, err))
        ).unwrap_or_else(|err| panic!("Could not parse move file at {:?} with error {}", ron_path, err))
    } else {
        SerializedMove {
            pokemon_move: from_file(toml_path),
            game_move: game_move(ron_path),
        }
    }
}

fn from_file(path: PathBuf) -> Move {
    toml::from_str(
        &read_to_string(&path)
            .unwrap_or_else(|err| panic!("Could not read move file at {:?} to string with error {}", path, err))
    ).unwrap_or_else(|err| panic!("Could not parse move file at {:?} with error {}", path, err))
}

fn game_move(path: PathBuf) -> Option<GamePokemonMove> {
    if path.exists() {
        Some(
            ron::from_str(
                &read_to_string(&path)
                    .unwrap_or_else(|err| panic!("Could not read game move file at {:?} to string with error {}", path, err))
            ).unwrap_or_else(|err| panic!("Could not parse game move file at {:?} with error {}", path, err))
        )
    } else {
        None
    }
}