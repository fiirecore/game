use std::{path::Path, fs::{read_dir, read_to_string, read}};

use firecore_world::audio::MusicId;
use hashbrown::HashMap;
use serde::Deserialize;

use crate::{readable, write};

pub fn build(root: impl AsRef<Path>, assets: &Path) {

    if readable::<HashMap<MusicId, Vec<u8>>, _>(&root, "music").is_none() {
        write(&root, "music", build_music(assets.join("music/")));
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MusicData<D> {
    pub id: MusicId,
    pub data: D,
}

fn build_music(music_folder: impl AsRef<Path>) -> HashMap<MusicId, Vec<u8>> {
    let e = std::ffi::OsString::from("ron");

    let music_folder = music_folder.as_ref();

    let extension = Some(e.as_os_str());

    read_dir(music_folder)
        .unwrap_or_else(|err| panic!("Could not read music directory with error {}", err))
        .flat_map(|dir| match dir.map(|dir| dir.path()) {
            Ok(path) => {
                if path.is_file() && path.extension() == extension {
                    let content = read_to_string(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not read file at {:?} to string with error {}",
                            path, err
                        )
                    });
                    let data: MusicData<String> = ron::from_str(&content).unwrap_or_else(|err| {
                        panic!(
                            "Could not deserialize file at {:?} with error {}",
                            path, err
                        )
                    });
                    let music_bytes = read(music_folder.join(&data.data)).unwrap_or_else(|err| {
                        panic!(
                            "Could not get music file for {} with error {}",
                            data.id, err
                        )
                    });

                    Some((data.id, music_bytes))
                } else {
                    None
                }
            }
            Err(err) => {
                eprintln!(
                    "Could not read entry under music directory with error {}",
                    err
                );
                None
            }
        })
        .collect()
}
