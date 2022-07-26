use std::{
    fs::{read, read_dir, read_to_string},
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
};

use hashbrown::HashMap;

use firecore_dex_gen::{moves::Execution, pokemon::PokemonOutput};
use firecore_world::{
    character::{npc::group::NpcGroup, Activity, CharacterGroupId},
    map::{
        data::{tile::PaletteDataMap, WorldMapData, WorldNpcData},
        wild::WildChances,
        PaletteId,
    },
    positions::Spot,
    script::default::DefaultWorldScriptEngine,
    serialized::{
        SerializedCharacterGroupTextures, SerializedPalette, SerializedPlayerTexture,
        SerializedTextures,
    }, pokedex::{item::{ItemId, Item}, moves::{MoveId, Move}, pokemon::Pokemon},
};
use serde::{Serialize, Deserialize};

const POKEMON: Range<i16> = 1..386;
const MOVES: Range<i16> = 1..559;

fn main() {
    // println!("cargo:rerun-if-changed=assets");

    let root = Path::new("data/");

    if !root.exists() {
        std::fs::create_dir_all(root).unwrap();
    }

    #[cfg(feature = "audio")]
    write("audio", &firecore_audio_builder::compile("assets/music"));

    let client = firecore_dex_gen::client();

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

    if readable::<SerializedPokedexEngine, _>(&root, "dex_engine").is_none() {

        let dex_engine = SerializedPokedexEngine {
            pokemon: firecore_dex_gen::pokemon::generate_client(&pokemon),
            items: firecore_dex_gen::items::generate_client(client.clone()),
            trainer_groups: get_npc_groups("assets/world/trainers"),
        };

        write(&root, "dex_engine", &dex_engine);
    }

    if readable::<WorldMapData, _>(&root, "world").is_none()
        || readable::<DefaultWorldScriptEngine, _>(&root, "world_script").is_none()
        || readable::<SerializedTextures, _>(&root, "world_textures").is_none()
    {
        let mappings =
            ron::from_str(&read_to_string("assets/world/mappings.ron").unwrap()).unwrap();
        let edits = ron::from_str(&read_to_string("assets/world/edits.ron").unwrap()).unwrap();

        let data = firecore_world_gen::create_data().unwrap();

        let firecore_world_gen::WorldData { maps, scripts } =
            firecore_world_gen::compile(mappings, edits, data).unwrap();

        let (npc, npcs) = load_npc_groups("assets/world/npcs", "assets/world/trainers");

        let BuilderWorldData {
            palettes,
            wild,
            spawn,
        } = ron::from_str(&read_to_string("assets/world/data.ron").unwrap()).unwrap();

        let world = WorldMapData {
            maps,
            palettes,
            npc,
            wild,
            spawn,
        };

        let textures = SerializedTextures {
            palettes: palette("assets/world/textures/palettes"),
            npcs,
            objects: Default::default(),
            player: player("assets/world/textures/player"),
        };

        write(&root, "world", &world);
        write(&root, "world_script", &scripts);
        write(&root, "world_textures", &textures);
    }

    if readable::<Execution, _>(&root, "battle_moves").is_none() {
        write(
            &root,
            "battle_moves",
            &firecore_dex_gen::moves::generate_battle(client, MOVES),
        );
    }

    if readable::<Scripts, _>(&root, "battle_move_scripts").is_none() {
        write(
            &root,
            "battle_move_scripts",
            get_moves("assets/battle/moves/scripts"),
        );
    }

    #[cfg(windows)]
    if std::env::var("PROFILE").unwrap() == "release" && std::env::var("CARGO_CFG_WINDOWS").is_ok()
    {
        embed_resource::compile("assets/resource.rc");
    }
}

// fn directory() -> std::path::PathBuf {
//     let dest = firecore_storage::directory(false, PUBLISHER, APPLICATION)
//         .unwrap()
//         .join("assets");
//     if !dest.exists() {
//         std::fs::create_dir_all(&dest).unwrap();
//     }
//     dest
// }

fn readable<S: serde::de::DeserializeOwned, P: AsRef<std::path::Path>>(
    root: P,
    file: &str,
) -> Option<S> {
    let file = match read(root.as_ref().join(format!("{}.bin", file))) {
        Ok(file) => file,
        Err(..) => return None,
    };
    postcard::from_bytes::<S>(&file).ok()
}

fn write<S: serde::Serialize>(root: impl AsRef<std::path::Path>, file: &str, data: S) -> S {
    std::fs::write(
        root.as_ref().join(format!("{}.bin", file)),
        &postcard::to_allocvec(&data).unwrap(),
    )
    .unwrap_or_else(|_| panic!("Cannot make path for {}", file));
    data
}

pub type TrainerGroupId = CharacterGroupId;

pub type TrainerGroupOutput = HashMap<TrainerGroupId, Vec<u8>>;

pub const NAME: &str = "Trainer Group";

pub fn get_npc_groups(path: impl AsRef<std::path::Path>) -> TrainerGroupOutput {
    std::fs::read_dir(path)
        .unwrap_or_else(|err| panic!("Could not read {} directory with error {}", NAME, err))
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|path| {
            (
                path.file_stem()
                    .unwrap_or_else(|| panic!("Could not get filename for {} at {:?}", NAME, path))
                    .to_string_lossy()
                    .parse()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Cannot parse file name for {} at {:?} with error {}",
                            NAME, path, err
                        )
                    }),
                std::fs::read(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read {} entry at {:?} with error {}",
                        NAME, path, err
                    )
                }),
            )
        })
        .collect()
}

fn palette(path: impl AsRef<Path>) -> hashbrown::HashMap<PaletteId, SerializedPalette> {
    read_dir(path.as_ref())
        .unwrap_or_else(|err| panic!("Could not read tile palette folder with error {}", err))
        .flatten()
        .map(|entry| entry.path())
        .map(|path| match path.is_file() {
            true => {
                let filename = crate::filename(&path);
                let id = filename[7..filename.len() - 1]
                    .parse::<PaletteId>()
                    .unwrap_or_else(|err| {
                        panic!("Could not read palette id at {:?} with error {}", path, err)
                    });

                let texture = read(&path).unwrap_or_else(|err| {
                    panic!("Could not read palette #{} with error {}", id, err)
                });

                (
                    id,
                    SerializedPalette {
                        texture,
                        animated: Default::default(),
                        doors: Default::default(),
                    },
                )
            }
            false => {
                let id = crate::filename(&path)
                    .parse::<PaletteId>()
                    .unwrap_or_else(|err| {
                        panic!("Could not read palette id at {:?} with error {}", path, err)
                    });

                let palette = path.join("palette.png");

                let texture = read(&palette).unwrap_or_else(|err| {
                    panic!("Could not read palette #{} with error {}", id, err)
                });

                let animated = read_folder(path.join("animated"));

                let doors = read_folder(path.join("doors"));

                (
                    id,
                    SerializedPalette {
                        texture,
                        animated,
                        doors,
                    },
                )
            }
        })
        .collect()
}

fn filename(path: &std::path::Path) -> String {
    path.file_stem()
        .map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(|| panic!("Could not read the file stem of file at {:?}", path))
}

fn read_folder<I: std::hash::Hash + Eq + FromStr<Err = E>, E: core::fmt::Display>(
    path: impl AsRef<Path>,
) -> hashbrown::HashMap<I, Vec<u8>> {
    let path = path.as_ref();
    read_dir(path)
        .into_iter()
        .flat_map(|rd| rd.into_iter().flatten())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .map(|p| {
            let id = crate::filename(&p).parse().unwrap_or_else(|err| {
                panic!(
                    "Could not get tile id for animated texture from file name {:?} with error {}",
                    path, err
                )
            });

            let texture = read(&p).unwrap_or_else(|err| {
                panic!(
                    "Could not read animated texture at {:?} with error {}",
                    p, err
                )
            });

            (id, texture)
        })
        .collect()
}

pub fn load_npc_groups(
    npcs: impl AsRef<Path>,
    trainers: impl AsRef<Path>,
) -> (WorldNpcData, SerializedCharacterGroupTextures) {
    let npc_dir = npcs.as_ref();
    let trainer_dir = trainers.as_ref();
    let mut npcs = HashMap::new();
    let mut trainers = HashMap::new();
    let mut textures = HashMap::new();

    for entry in read_dir(npc_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not get NPC group directory at {:?} with error {}",
                npc_dir, err
            )
        })
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!(
                    "Could not directory entry under {:?} with error {}",
                    npc_dir, err
                )
            })
        })
    {
        let path = entry.path();
        if path.is_dir() {
            let ron_path = get_npc_type_file(&path);

            let id = crate::filename(&ron_path);

            let group: NpcGroup = ron::from_str(&read_to_string(&ron_path).unwrap_or_else(|err| {
                panic!(
                    "Could not get Npc type file at {:?} with error {}",
                    ron_path, err
                )
            }))
            .unwrap_or_else(|err| {
                panic!(
                    "Could not decode Npc type file at {:?} with error {}",
                    ron_path, err
                )
            });

            let id1 = id.parse().unwrap_or_else(|err| {
                panic!(
                    "Cannot parse npc group file name {} into id with error {}",
                    id, err
                )
            });

            let sprite_path = path.join(id + ".png");
            let texture = read(&sprite_path).unwrap_or_else(|err| {
                panic!(
                    "Could not get npc sprite at {:?} with error {}",
                    sprite_path, err
                )
            });

            npcs.insert(id1, group);
            textures.insert(id1, texture);
        }
    }

    for path in read_dir(&trainer_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not get trainer group directory at {:?} with error {}",
                trainer_dir, err
            )
        })
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!(
                    "Could not directory entry under {:?} with error {}",
                    trainer_dir, err
                )
            })
        })
        .map(|d| d.path())
    {
        let id = crate::filename(&path).parse().unwrap_or_else(|err| {
            panic!(
                "Could not parse file name at {:?} into trainer group id with error {}",
                path, err
            )
        });

        let trainer = ron::from_str(&read_to_string(&path).unwrap_or_else(|err| {
            panic!(
                "Could not read trainer group entry at {:?} with error {}",
                path, err
            )
        }))
        .unwrap_or_else(|err| {
            panic!(
                "Could not deserialize trainer group at {:?} with error {}",
                path, err
            )
        });

        trainers.insert(id, trainer);
    }

    (
        WorldNpcData {
            groups: npcs,
            trainers,
        },
        textures,
    )
}

fn get_npc_type_file(path: &Path) -> PathBuf {
    for entry in read_dir(path).unwrap().flatten() {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == std::ffi::OsString::from("ron") {
                return path;
            }
        }
    }
    panic!("Could not find Npc type under folder {:?}", path);
}

fn player(path: impl AsRef<Path>) -> SerializedPlayerTexture {
    let path = path.as_ref();
    firecore_world::serialized::enum_map::enum_map! {
        Activity::Walking => read(path.join("walking.png"))
            .unwrap_or_else(|err| panic!("Cannot read player walking texture with error {}", err)),
            Activity::Running => read(path.join("running.png"))
            .unwrap_or_else(|err| panic!("Cannot read player running texture with error {}", err)),
            Activity::Swimming => read(path.join("swimming.png"))
            .unwrap_or_else(|err| panic!("Cannot read player swimming texture with error {}", err)),
    }
}

type Scripts = std::collections::HashMap<MoveId, String>;

fn get_moves<P: AsRef<Path>>(scripts: P) -> Scripts {
    let scripts = scripts.as_ref();

    read_dir(scripts)
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
                read_to_string(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read move file at {:?} to string with error {}",
                        path, err
                    )
                }),
            )
        })
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct BuilderWorldData {
    pub palettes: PaletteDataMap,
    pub wild: WildChances,
    pub spawn: Spot,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedPokedexEngine {
    pokemon: PokemonOutput,
    items: HashMap<ItemId, Vec<u8>>,
    trainer_groups: TrainerGroupOutput,
}