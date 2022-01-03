use firecore_pokedex::item::ItemId;
use hashbrown::HashMap;

pub type ItemOutput = HashMap<ItemId, Vec<u8>>;

#[cfg(feature = "compile")]
pub fn get_items<P: AsRef<std::path::Path>>(path: P) -> ItemOutput {
    let path = path.as_ref();
    std::fs::read_dir(path)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read item textures directory at {:?} with error {}",
                path, err
            )
        })
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .map(|path| {
            let texture = std::fs::read(&path).unwrap_or_else(|err| {
                panic!(
                    "Could not read item texture file at {:?} with error {}",
                    path, err
                )
            });
            let id: ItemId = path
                .file_stem()
                .unwrap_or_else(|| panic!("Cannot get file name for file at {:?}", path))
                .to_string_lossy()
                .parse()
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not get ItemId from filename {:?} with error {}",
                        path, err
                    )
                });
            (id, texture)
        })
        .collect()
}
