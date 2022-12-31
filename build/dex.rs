use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use firecore_dex_gen::Client;
use firecore_pokedex_client_data::pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use crate::{readable, write, MOVES, POKEMON};

const POKEDEX: &str = "pokedex";

pub fn build(root: impl AsRef<Path>, assets: &Path) -> (Client, Vec<Pokemon>) {
    let root = root.as_ref();

    let client = firecore_dex_gen::client();

    let pokemon = match readable::<Vec<Pokemon>, _>(&root, POKEDEX) {
        Some(p) => p,
        None => write(
            &root,
            POKEDEX,
            firecore_dex_gen::pokemon::generate(client.clone(), POKEMON),
        ),
    };

    let (moves, bmoves) = firecore_dex_gen::moves::generate(client.clone(), MOVES).into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    match readable::<Vec<Move>, _>(&root, "movedex") {
        Some(m) => m,
        None => write(
            &root,
            "movedex",
            moves,
        ),
    };

    match readable::<Vec<_>, _>(&root, "battle_moves") {
        Some(m) => m,
        None => write(
            &root,
            "battle_moves",
            bmoves,
        ),
    };

    // match readable::<Vec<Item>, _>(&root, "itemdex") {
    //     Some(i) => i,
        // None => {
            let mut items1 = firecore_dex_gen::items::generate();
            let items2 = build_items(assets).unwrap();
            items1.extend(items2);
            write(&root, "itemdex", items1);
    //     }
    // };

    (client, pokemon)
}

fn build_items(assets: &Path) -> Result<Vec<Item>, std::io::Error> {
    Ok(read_dir(assets.join("dex/items"))?
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|p| ron::from_str::<Item>(&read_to_string(&p).unwrap()).unwrap())
        .collect())
}
