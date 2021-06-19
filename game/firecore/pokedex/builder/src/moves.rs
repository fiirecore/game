use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use pokedex::{
    battle::serialized::SerializedBattleMoveFile, moves::Move, serialize::SerializedMove,
};

pub fn get_moves<P: AsRef<std::path::Path>>(move_dir: P) -> Vec<SerializedMove> {
    let move_dir = move_dir.as_ref();
    read_dir(move_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read moves directory at {:?} with error {}",
                move_dir, err
            )
        })
        .map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => Some(if path.is_dir() {
                from_dir(path)
            } else {
                SerializedMove::from(from_file(path))
            }),
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            }
        })
        .flatten()
        .collect()
}

fn from_dir(dir: PathBuf) -> SerializedMove {
    let move_path = dir.join("move.ron");
    let battle_path = dir.join("battle.ron");

    SerializedMove {
        pokemon_move: ron::from_str::<Move>(&read_to_string(&move_path).unwrap_or_else(|err| {
            panic!(
                "Could not read move file at {:?} to string with error {}",
                move_path, err
            )
        }))
        .unwrap_or_else(|err| {
            panic!(
                "Could not parse move file at {:?} with error {}",
                move_path, err
            )
        }),
        battle_move: read_to_string(&battle_path).ok().map(|data| {
            ron::from_str::<SerializedBattleMoveFile>(&data)
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not parse move battle file at {:?} with error {}",
                        battle_path, err
                    )
                })
                .into(dir)
        }),
    }
}

fn from_file(path: PathBuf) -> Move {
    ron::from_str(&read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "Could not read move file at {:?} to string with error {}",
            path, err
        )
    }))
    .unwrap_or_else(|err| panic!("Could not parse move file at {:?} with error {}", path, err))
}
