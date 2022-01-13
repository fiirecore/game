use serde::Deserialize;

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;
pub type SoundVariant = Option<u16>;

use std::{
    fs::{read, read_dir, read_to_string},
    path::Path,
};

#[derive(Debug, Clone, Deserialize)]
pub struct MusicData<D> {
    pub id: MusicId,
    pub data: D,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SoundData<D> {
    pub id: SoundId,
    #[serde(default)]
    pub variant: SoundVariant,
    pub data: D,
}

type Map<K> = std::collections::HashMap<K, Vec<u8>>;
type M = Map<MusicId>;

pub fn compile(music: impl AsRef<Path>) -> M {
    let music_folder = music.as_ref();

    let mut music = M::new();

    let e = std::ffi::OsString::from("ron");

    let extension = Some(e.as_os_str());

    for dir in read_dir(music_folder)
        .unwrap_or_else(|err| panic!("Could not read music directory with error {}", err))
    {
        match dir.map(|dir| dir.path()) {
            Ok(path) => {
                if path.is_file() && path.extension() == extension {
                    let content = read_to_string(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not read file at {:?} to string with error {}",
                            path, err
                        )
                    });
                    let data: MusicData<String> =
                        ron::from_str(&content).unwrap_or_else(|err| {
                            panic!(
                                "Could not deserialize file at {:?} with error {}",
                                path, err
                            )
                        });
                    let music_bytes =
                        read(music_folder.join(&data.data)).unwrap_or_else(|err| {
                            panic!(
                                "Could not get music file for {} with error {}",
                                data.id, err
                            )
                        });

                    if let Some(..) = music.insert(data.id,music_bytes) {
                        panic!("Duplicate tracks found with id {}!", data.id);
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "Could not read entry under music directory with error {}",
                    err
                );
            }
        }
    }

    // if let Some(parent) = Path::new(output_file).parent() {
    //     if !parent.exists() {
    //         create_dir_all(parent).unwrap_or_else(|err| panic!("Could not create directory for output file with error {}", err));
    //     }
    // }

    // File::create(output_file)
    //     .unwrap_or_else(|err| panic!("Could not create output file with error {}", err))
    //         .write_all(&firecore_dependencies::ser::serialize(&data).unwrap_or_else(|err| panic!("Could not serialize audio data with error {}", err)))
    //             .unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));

    music
}
