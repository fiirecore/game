use std::{
    fs::{read, read_dir},
    hash::Hash,
    path::{Path, PathBuf},
    str::FromStr,
};

use hashbrown::HashMap;

use world::{
    character::MovementType,
    map::PaletteId,
    serialized::{SerializedPalette, SerializedPlayerTexture, SerializedTextures},
};

// #[derive(serde::Deserialize)]
// pub struct SerializedDoor {
//     pub tile: TileId,
//     pub file: String,
// }

pub fn get_textures<P: AsRef<Path>>(textures: P) -> SerializedTextures {
    let textures = textures.as_ref();
    SerializedTextures {
        palettes: read_dir(textures.join("palettes"))
            .unwrap_or_else(|err| panic!("Could not read tile palette folder with error {}", err))
            .flatten()
            .map(|entry| entry.path())
            .map(palette)
            .collect(),
        // animated: read_dir(textures.join("animated"))
        //     .unwrap_or_else(|err| {
        //         panic!("Could not read animated tiles directory with error {}", err)
        //     })
        //     .flatten()
        //     .map(|entry| entry.path())
        //     .filter(|path| )
        //     .map(|path| {
        //         let filename = filename(&path);
        //         let id = filename.parse::<TileId>().unwrap_or_else(|err| {
        //             panic!("Could not read animated texture tile ID with error {}", err)
        //         });
        //         (
        //             id,
        //             read(&path).unwrap_or_else(|err| {
        //                 panic!(
        //                     "Could not read animated texture with ID {} with error {}",
        //                     id, err
        //                 )
        //             }),
        //         )
        //     })
        //     .collect(),
        // doors:,
        player: player(textures.join("player")),
        npcs: Default::default(),
        objects: read_folder(textures.join("objects")),
    }
}

fn palette(path: PathBuf) -> (PaletteId, SerializedPalette) {
    match path.is_file() {
        true => {
            let filename = crate::filename(&path);
            let id = filename[7..filename.len() - 1]
                .parse::<PaletteId>()
                .unwrap_or_else(|err| {
                    panic!("Could not read palette id at {:?} with error {}", path, err)
                });

            let texture = read(&path)
                .unwrap_or_else(|err| panic!("Could not read palette #{} with error {}", id, err));

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

            let texture = read(&palette)
                .unwrap_or_else(|err| panic!("Could not read palette #{} with error {}", id, err));

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
    }
}

fn read_folder<I: Hash + Eq + FromStr<Err = E>, E: core::fmt::Display>(
    path: impl AsRef<Path>,
) -> HashMap<I, Vec<u8>> {
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

fn player(path: PathBuf) -> SerializedPlayerTexture {
    enum_map::enum_map! {
        MovementType::Walking => read(path.join("walking.png"))
            .unwrap_or_else(|err| panic!("Cannot read player walking texture with error {}", err)),
        MovementType::Running => read(path.join("running.png"))
            .unwrap_or_else(|err| panic!("Cannot read player running texture with error {}", err)),
        MovementType::Swimming => read(path.join("swimming.png"))
            .unwrap_or_else(|err| panic!("Cannot read player swimming texture with error {}", err)),
    }
}

// fn doors(path: PathBuf) -> SerializedDoors {
//     // let mut doors = SerializedDoors::new();
//     // let extension = Some(std::ffi::OsStr::new("ron"));
//     // for path in read_dir(path).unwrap().flatten().map(|entry| entry.path()) {
//     //     if path.is_file() && path.extension() == extension {
//     //         let door =
//     //             ron::from_str::<SerializedDoor>(&read_to_string(&path).unwrap_or_else(|err| {
//     //                 panic!(
//     //                     "Could not get door file from path {:?} with error {}",
//     //                     path, err
//     //                 )
//     //             }))
//     //             .unwrap();
//     //         let path = path.parent().unwrap().join(door.file);
//     //         doors.insert(
//     //             door.tiles,
//     //             read(&path).unwrap_or_else(|err| {
//     //                 panic!(
//     //                     "Could not read door image file at {:?} with error {}",
//     //                     path, err
//     //                 )
//     //             }),
//     //         );
//     //     }
//     // }
//     // doors
// }
