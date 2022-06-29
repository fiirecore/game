use enum_map::{enum_map, Enum, EnumMap};
use firecore_pokedex::pokemon::{PokemonId, PokemonTexture};
use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

pub type SerializedPokemon = (EnumMap<PokemonTexture, Vec<u8>>, Vec<u8>);

pub type PokemonOutput = HashMap<PokemonId, SerializedPokemon>;

#[cfg(feature = "compile")]
pub fn get_pokemon<P: AsRef<std::path::Path>>(path: P) -> PokemonOutput {
    let path = path.as_ref();

    let readdir = std::fs::read_dir(path).unwrap_or_else(|err| {
        panic!(
            "Could not read pokemon directory at {:?} with error {}",
            path, err
        )
    });

    readdir
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_dir())
        .map(|path| {
            // to - do: override default id by checking folder for "id.txt"

            let id = path
                .file_name()
                .unwrap_or_else(|| panic!("Could not get directory name of path {:?}", path))
                .to_string_lossy()
                .parse()
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not get PokemonId from directory named {:?} with error {}",
                        path, err
                    )
                });

            let textures = enum_map! {
                PokemonTexture::Front => {
                    let path = path.join("front.png");
                    std::fs::read(&path).unwrap_or_else(|err| {
                        panic!(
                    "Could not get front texture for pokemon at path {:?} with error {}",
                    path, err
                )
                    })
                },
                PokemonTexture::Back => {
                    let path = path.join("back.png");
                    std::fs::read(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not get back texture for pokemon at path {:?} with error {}",
                            path, err
                        )
                    })
                },
                PokemonTexture::Icon => {
                    let path = path.join("icon.png");
                    std::fs::read(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not get icon texture for pokemon at path {:?} with error {}",
                            path, err
                        )
                    })
                },
            };

            let cry = cfg!(feature = "audio")
                .then(|| {
                    let path = path.join("cry.ogg");
                    std::fs::read(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not get back texture for pokemon at path {:?} with error {}",
                            path, err
                        )
                    })
                })
                .unwrap_or_default();

            (id, (textures, cry))
        })
        .collect()
}
