use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use pokedex::moves::Move;
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
    let file = dir.join("move.ron");
    
    ron::from_str::<SerializedMove>(
        &read_to_string(&file)
            .unwrap_or_else(|err| panic!("Could not read move file at {:?} to string with error {}", file, err))
    ).unwrap_or_else(|err| panic!("Could not parse move file at {:?} with error {}", file, err))

    // let wasm = dir.join("plugin.wasm");

    // if wasm.exists() {
    //     let game_move = ser.game_move.get_or_insert(Default::default());
    //     match read(wasm) {
    //         Ok(bytes) => game_move.plugin = Some(bytes),
    //         Err(err) => panic!("Could not read wasm plugin for move {} with error {}", ser.pokemon_move.name, err),
    //     }
    // }

    // ser

    // if !path.exists() {
    // } else {
    //     SerializedMove {
    //         pokemon_move: from_file(path),
    //         game_move: game_move(ron_path),
    //     }
    // }
}

fn from_file(path: PathBuf) -> Move {
    ron::from_str(
        &read_to_string(&path)
            .unwrap_or_else(|err| panic!("Could not read move file at {:?} to string with error {}", path, err))
    ).unwrap_or_else(|err| panic!("Could not parse move file at {:?} with error {}", path, err))
}

// fn game_move(path: PathBuf) -> Option<GamePokemonMove> {
//     if path.exists() {
//         Some(
//             ron::from_str(
//                 &read_to_string(&path)
//                     .unwrap_or_else(|err| panic!("Could not read game move file at {:?} to string with error {}", path, err))
//             ).unwrap_or_else(|err| panic!("Could not parse game move file at {:?} with error {}", path, err))
//         )
//     } else {
//         None
//     }
// }