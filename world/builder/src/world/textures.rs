use std::{fs::{read_dir, read, read_to_string}, ffi::OsStr};
use worldlib::serialized::{Doors, SerializedDoor, SerializedTextures};
use worldlib::map::{TileId, PaletteId};
use std::path::{Path, PathBuf};

pub fn get_textures<P: AsRef<Path>>(textures: P, extension: Option<&OsStr>) -> SerializedTextures {
    let textures = textures.as_ref();
    SerializedTextures {
        palettes: read_dir(textures.join("palettes"))
            .unwrap_or_else(|err| panic!("Could not read tile palette folder with error {}", err))
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .map(|path| {
                let filename = filename(&path);
                let id = filename[7..filename.len()-1].parse::<PaletteId>()
                    .unwrap_or_else(|err| panic!("Could not read palette id at {:?} with error {}", path, err));
                (
                    id,
                    read(&path)
                        .unwrap_or_else(|err| panic!("Could not read palette #{} with error {}", id, err))
                )
            })
            .collect(),
        animated: read_dir(textures.join("animated"))
            .unwrap_or_else(|err| panic!("Could not read animated tiles directory with error {}", err))
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .map(|path| {
                let filename = filename(&path);
                let id = filename.parse::<TileId>()
                    .unwrap_or_else(|err| panic!("Could not read animated texture tile ID with error {}", err));
                (
                    id,
                    read(&path)
                        .unwrap_or_else(|err| panic!("Could not read animated texture with ID {} with error {}", id, err))
                )
            })
            .collect(),
        doors: doors(textures.join("doors"), extension)
    }
}

fn doors(path: PathBuf, extension: Option<&OsStr>) -> Doors {
    let mut doors = Doors::new();
    for path in read_dir(path).unwrap().flatten().map(|entry| entry.path()) {
        if path.is_file() && path.extension() == extension {
            let door = ron::from_str::<SerializedDoor>(
                &read_to_string(&path).unwrap_or_else(|err| panic!("Could not get door file from path {:?} with error {}", path, err))
            ).unwrap();
            let path = path.parent().unwrap().join(door.file);
            doors.insert(door.tiles, read(&path).unwrap_or_else(|err| panic!("Could not read door image file at {:?} with error {}", path, err)));
        }
    }
    doors
}

fn filename(path: &Path) -> String {
    path.file_stem().map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(|| panic!("Could not read the file stem of file at {:?}", path))
}