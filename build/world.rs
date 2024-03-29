use std::{
    fs::{read, read_dir, read_to_string},
    path::{Path, PathBuf},
    str::FromStr,
};

use firecore_world::{
    character::{npc::group::NpcGroup, Activity},
    map::{
        data::{tile::PaletteDataMap, FieldItemData, FieldMoveData, WorldMapData, WorldNpcData},
        wild::WildChances,
        PaletteId,
    },
    positions::Spot,
    script::default::DefaultWorldScriptEngine,
    serialized::{
        SerializedCharacterGroupTextures, SerializedPalette, SerializedPlayerTexture,
        SerializedTextures,
    },
};
use hashbrown::HashMap;

use crate::{readable, write};

pub fn build(root: impl AsRef<Path>, assets: &Path) {
    if readable::<WorldMapData, _>(&root, "world").is_none()
        || readable::<DefaultWorldScriptEngine, _>(&root, "world_script").is_none()
        || readable::<SerializedTextures, _>(&root, "world_textures").is_none()
    {
        let mappings =
            ron::from_str(&read_to_string(assets.join("world/mappings.ron")).unwrap()).unwrap();
        let edits =
            ron::from_str(&read_to_string(assets.join("world/edits.ron")).unwrap()).unwrap();

        let data = firecore_world_gen::create_data().unwrap();

        let firecore_world_gen::WorldData { maps, scripts } =
            firecore_world_gen::compile(mappings, edits, data).unwrap();

        let (npc, npcs) = load_npc_groups(assets.join("world/npcs"), assets.join("world/trainers"));

        let BuilderWorldData {
            palettes,
            wild,
            moves,
            items,
            spawn,
        } = ron::from_str(&read_to_string(assets.join("world/data.ron")).unwrap()).unwrap();

        let world = WorldMapData {
            maps,
            palettes,
            npc,
            wild,
            spawn,
            moves,
            items,
        };

        let textures = SerializedTextures {
            palettes: palette(assets.join("world/textures/palettes")),
            npcs,
            objects: Default::default(),
            player: player(assets.join("world/textures/player")),
        };

        write(&root, "world", &world);
        write(&root, "world_script", &scripts);
        write(&root, "world_textures", &textures);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BuilderWorldData {
    pub palettes: PaletteDataMap,
    pub wild: WildChances,
    pub moves: FieldMoveData,
    pub items: FieldItemData,
    pub spawn: Spot,
}

fn palette(path: impl AsRef<Path>) -> hashbrown::HashMap<PaletteId, SerializedPalette> {
    read_dir(path.as_ref())
        .unwrap_or_else(|err| panic!("Could not read tile palette folder with error {}", err))
        .flatten()
        .map(|entry| entry.path())
        .map(|path| match path.is_file() {
            true => {
                let filename = filename(&path);
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
                let id = filename(&path).parse::<PaletteId>().unwrap_or_else(|err| {
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

fn filename(path: &Path) -> String {
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
            let id = filename(&p).parse().unwrap_or_else(|err| {
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
    let mut textures = SerializedCharacterGroupTextures::new();

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

            let id = filename(&ron_path);

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
        let id = filename(&path).parse().unwrap_or_else(|err| {
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
            Activity::Cycling => read(path.join("bike.png"))
            .unwrap_or_else(|err| panic!("Cannot read player cycling texture with error {}", err)),
    }
}
