use std::{ops::Range, path::Path};

use firecore_dex_gen::{moves::Execution, pokemon::PokemonOutput};
use firecore_pokedex_engine_builder::pokedex::moves::MoveId;
use firecore_world_builder::world::{script::default::DefaultWorldScriptEngine, map::manager::WorldMapData, serialized::SerializedTextures};

fn directory() -> std::path::PathBuf {
    let dest = firecore_storage::directory(false, PUBLISHER, APPLICATION)
        .unwrap()
        .join("assets");
    if !dest.exists() {
        std::fs::create_dir_all(&dest).unwrap();
    }
    dest
}

fn readable<S: serde::de::DeserializeOwned, P: AsRef<std::path::Path>>(
    root: P,
    file: &str,
) -> Option<S> {
    let file = match std::fs::read(root.as_ref().join(format!("{}.bin", file))) {
        Ok(file) => file,
        Err(..) => return None,
    };
    firecore_storage::from_bytes::<S>(&file).ok()
}

fn write<S: serde::Serialize>(root: impl AsRef<std::path::Path>, file: &str, data: S) -> S {
    std::fs::write(
        root.as_ref().join(format!("{}.bin", file)),
        firecore_storage::to_bytes(&data).unwrap(),
    )
    .unwrap();
    data
}

const PUBLISHER: Option<&str> = Some("fiirecore");
const APPLICATION: &str = env!("CARGO_PKG_NAME");

const POKEMON: Range<i16> = 1..386;
const MOVES: Range<i16> = 1..559;

fn main() {
    // println!("cargo:rerun-if-changed=assets");

    if std::fs::read_dir("assets/game")
        .map(|d| d.count())
        .unwrap_or_default()
        == 0
    {
        std::process::Command::new("git")
            .args(["submodule", "init"])
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not initialize git submodules using git command with error {}",
                    err
                )
            })
            .wait()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not wait on git submodule init task to complete with error {}",
                    err
                )
            });

        std::process::Command::new("git")
            .args(["submodule", "update", "--remote"])
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Could not update git submodules using git command with error {}",
                    err
                )
            }).wait().unwrap_or_else(|err| {
                panic!(
                    "Could not wait on git submodule update --remote task to complete with error {}",
                    err
                )
            });
    }

    let root = directory();

    #[cfg(feature = "audio")]
    write("audio", &firecore_audio_builder::compile("assets/music"));

    let client = firecore_dex_gen::client();

    use firecore_pokedex_engine_builder::pokedex::{item::Item, moves::Move, pokemon::Pokemon};

    let pokemon = match readable::<Vec<Pokemon>, _>(&root, "pokedex") {
        Some(p) => p,
        None => write(
            &root,
            "pokedex",
            firecore_dex_gen::pokemon::generate(client.clone(), POKEMON),
        ),
    };

    let _ = match readable::<Vec<Move>, _>(&root, "movedex") {
        Some(m) => m,
        None => write(
            &root,
            "movedex",
            firecore_dex_gen::moves::generate(client.clone(), MOVES),
        ),
    };

    let _ = match readable::<Vec<Item>, _>(&root, "itemdex") {
        Some(i) => i,
        None => write(&root, "itemdex", firecore_dex_gen::items::generate()),
    };

    // if readable::<PokemonOutput, _>(&root, "pokemon_textures").is_none() {
    //     write(&root, "pokemon_textures", firecore_dex_gen::pokemon::generate_client(&pokemon));
    // }

    #[deprecated]
    let dex_engine = firecore_pokedex_engine_builder::compile(
        "assets/game/dex/client/pokemon",
        "assets/game/dex/client/items",
        "assets/game/dex/client/trainers",
    );

    write(&root, "dex_engine", &dex_engine);

    let (world, tex, script) = firecore_world_builder::compile("assets/world");

    if readable::<WorldMapData, _>(&root, "world").is_none() {
        write(&root, "world", &world);
    }

    if readable::<DefaultWorldScriptEngine, _>(&root, "world_script").is_none() {
        write(&root, "world_script", &script);
    }

    if readable::<SerializedTextures, _>(&root, "world_textures").is_none() {
        write(&root, "world_textures", &tex);
    }

    if readable::<Execution, _>(&root, "battle_moves").is_none() {
        write(
            &root,
            "battle_moves",
            &firecore_dex_gen::moves::generate_battle(client, MOVES),
        );
    }

    if readable::<Scripts, _>(&root, "battle_move_scripts").is_none() {
        write(&root, "battle_move_scripts", get_moves("assets/game/dex/battle/moves/scripts"));
    }

    #[cfg(windows)]
    if std::env::var("PROFILE").unwrap() == "release" && std::env::var("CARGO_CFG_WINDOWS").is_ok()
    {
        embed_resource::compile("build/resource.rc");
    }
}

pub type Scripts = std::collections::HashMap<MoveId, String>;

pub fn get_moves<P: AsRef<Path>>(scripts: P) -> Scripts {
    let scripts = scripts.as_ref();

    std::fs::read_dir(scripts)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read move scripts directory at {:?} with error {}",
                scripts, err
            )
        })
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|path| {
            (
                path.file_stem()
                    .unwrap_or_else(|| {
                        panic!("Could not get file name for script file at path {:?}", path,)
                    })
                    .to_string_lossy()
                    .parse::<MoveId>()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Could not parse move script file {:?} into MoveScriptId with error {}",
                            path, err
                        )
                    }),
                std::fs::read_to_string(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read move file at {:?} to string with error {}",
                        path, err
                    )
                }),
            )
        })
        .collect()
}
