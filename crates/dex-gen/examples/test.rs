use firecore_battle::{
    moves::BattleMove,
    pokedex::{
        moves::Move,
        pokemon::{data::PokemonTexture, PokemonId},
    },
};

fn main() {
    let client = firecore_dex_gen::client();

    let pokemon = firecore_dex_gen::pokemon::generate(client.clone(), 1..386);

    let client_pokemon = firecore_dex_gen::pokemon::generate_client(&pokemon);

    image::load_from_memory(&client_pokemon[&PokemonId(2u16)].0[PokemonTexture::Front]).unwrap();

    let (moves, bmoves) = firecore_dex_gen::moves::generate(client.clone(), 1..559)
        .into_iter()
        .unzip::<Move, BattleMove, Vec<_>, Vec<_>>();

    let execution = firecore_dex_gen::moves::generate_battle(client.clone(), 1..559).unwrap();

    // let items = firecore_dex_gen::items::generate();

    // let item_textures = firecore_dex_gen::items::generate_client(client);

    println!("Pokemon: {}", pokemon.len());

    println!("Client Pokemon: {}", client_pokemon.len());

    println!("Moves: {}", moves.len());

    println!("Battle Moves: {}", execution.len());

    std::fs::create_dir_all("data").unwrap();

    // let pokemon_path = Path::new("data/pokedex.bin");

    // std::fs::create_dir_all(pokemon_path).unwrap();

    // for (index, (textures, cry)) in client_pokemon.into_iter() {
    //     // to - do: named folders
    //     let folder = format!("{}", index);

    //     let path = pokemon_path.join(folder);

    //     if !path.exists() {
    //         std::fs::create_dir(&path).unwrap();
    //     }

    //     for (texture, bytes) in textures.into_iter() {
    //         let file = match texture {
    //             PokemonTexture::Front => "front.png",
    //             PokemonTexture::Back => "back.png",
    //             PokemonTexture::Icon => "icon.png",
    //         };

    //         std::fs::write(path.join(file), bytes).unwrap();
    //     }

    //     if !cry.is_empty() {
    //         std::fs::write(path.join("cry.ogg"), cry).unwrap();
    //     }
    // }
    std::fs::write("data/pokedex.bin", firecore_storage::to_bytes(&pokemon).unwrap()).unwrap();

    // std::fs::create_dir_all("generated/battle/moves").unwrap();

    // for (id, exec) in execution.into_iter() {
    //     std::fs::write(
    //         format!(
    //             "generated/battle/moves/{}.ron",
    //             &moves.iter().find(|m| m.id == id).unwrap().name
    //         ),
    //         ron::ser::to_string_pretty(&exec, Default::default())
    //             .unwrap()
    //             .as_bytes(),
    //     )
    //     .unwrap();
    // }

    std::fs::write(
        "data/movedex.bin",
        firecore_storage::to_bytes(&moves).unwrap(),
    )
    .unwrap();

    std::fs::write("data/battle_moves.bin", firecore_storage::to_bytes(&bmoves).unwrap()).unwrap();
    std::fs::write("data/battle_move_execution.bin", firecore_storage::to_bytes(&execution).unwrap()).unwrap();

    // std::fs::create_dir_all("generated/items").unwrap();

    // for item in items.into_iter() {
    //     std::fs::write(
    //         format!("generated/items/{}.ron", item.id),
    //         ron::ser::to_string_pretty(&item, Default::default())
    //             .unwrap()
    //             .as_bytes(),
    //     )
    //     .unwrap();
    // }

    // std::fs::create_dir_all("generated/client/items").unwrap();

    // for (id, item) in item_textures {
    //     std::fs::write(format!("generated/client/items/{}.png", id), &item).unwrap();
    // }
}
