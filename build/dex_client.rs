use std::path::Path;

use firecore_dex_gen::Client;
use firecore_pokedex_client_data::{pokedex::pokemon::Pokemon, PokemonOutput, ItemOutput, TrainerGroupOutput};

use crate::{readable, write};

const POKEMON_TEXTURES: &str = "pokemon_textures";

pub fn build(root: impl AsRef<Path>, assets: &Path, client: Client, pokemon: Vec<Pokemon>) {

    if readable::<PokemonOutput, _>(&root, POKEMON_TEXTURES).is_none() {
        write::<PokemonOutput, _>(
            &root,
            POKEMON_TEXTURES,
            firecore_dex_gen::pokemon::generate_client(&pokemon),
        );
    }

    if readable::<ItemOutput, _>(&root, "item_textures").is_none() {
        write::<ItemOutput, _>(
            &root,
            "item_textures",
            firecore_dex_gen::items::generate_client(client),
        );
    }

    if readable::<TrainerGroupOutput, _>(&root, "trainer_textures").is_none() {
        write::<TrainerGroupOutput, _>(
            &root,
            "trainer_textures",
            get_npc_groups(assets.join("battle/trainers")),
        );
    }

}

pub fn get_npc_groups(path: impl AsRef<Path>) -> TrainerGroupOutput {
    std::fs::read_dir(path)
        .unwrap_or_else(|err| panic!("Could not read trainer group directory with error {err}"))
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|path| {
            (
                path.file_stem()
                    .unwrap_or_else(|| {
                        panic!("Could not get filename for trainer group at {path:?}")
                    })
                    .to_string_lossy()
                    .parse()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Cannot parse file name for trainer group at {path:?} with error {err}",
                        )
                    }),
                std::fs::read(&path).unwrap_or_else(|err| {
                    panic!("Could not read trainer group entry at {path:?} with error {err}",)
                }),
            )
        })
        .collect()
}