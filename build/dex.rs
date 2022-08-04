use std::path::Path;

use firecore_dex_gen::Client;
use firecore_pokedex_engine_core::pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use crate::{readable, write, MOVES, POKEMON};

const POKEDEX: &str = "pokedex";

pub fn build(root: impl AsRef<Path>) -> (Client, Vec<Pokemon>) {
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

    match readable::<Vec<Move>, _>(&root, "movedex") {
        Some(m) => m,
        None => write(
            &root,
            "movedex",
            firecore_dex_gen::moves::generate(client.clone(), MOVES),
        ),
    };

    match readable::<Vec<Item>, _>(&root, "itemdex") {
        Some(i) => i,
        None => write(&root, "itemdex", firecore_dex_gen::items::generate()),
    };

    (client, pokemon)
}
