use worldlib::serialized::SerializedTextures;
use worldlib::{PaletteId, TileId};
use std::path::Path;

pub fn get_textures<P: AsRef<Path>>(textures: P) -> SerializedTextures {
    SerializedTextures {
        palettes: std::fs::read_dir(textures.as_ref().join("palettes"))
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
                    std::fs::read(&path)
                        .unwrap_or_else(|err| panic!("Could not read palette #{} with error {}", id, err))
                )
            })
            .collect(),
        animated: std::fs::read_dir(textures.as_ref().join("animated"))
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
                    std::fs::read(&path)
                        .unwrap_or_else(|err| panic!("Could not read animated texture with ID {} with error {}", id, err))
                )
            })
            .collect(),
    }
}

fn filename(path: &std::path::PathBuf) -> String {
    path.file_stem().map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(|| panic!("Could not read the file stem of file at {:?}", path))
}