use std::{
    fs::read,
    ops::Range,
    path::{Path, PathBuf},
};

const POKEMON: Range<i16> = 1..386;
const MOVES: Range<i16> = 1..559;

mod dex;
mod dex_client;
mod world;
mod battle;
mod title;
mod music;

fn main() {
    // println!("cargo:rerun-if-changed=assets");

    let assets = Path::new("assets");

    let root = PathBuf::from(
        std::env::var("OUT_DIR")
            .unwrap_or_else(|err| panic!("Cannot get OUT_DIR with error {}", err)),
    );

    if !root.exists() {
        std::fs::create_dir_all(&root).unwrap();
    }

    let (client, pokemon) = dex::build(&root, assets);

    // dex_client::build(&root, &assets, client.clone(), pokemon);

    // world::build(&root, &assets);

    // battle::build(&root, &assets, client);

    // title::build(&root, assets);

    // music::build(&root, assets);

    // #[cfg(windows)]
    // if std::env::var("PROFILE").unwrap() == "release" && std::env::var("CARGO_CFG_WINDOWS").is_ok()
    // {
    //     embed_resource::compile(assets.join("resource.rc"));
    // }
}

fn readable<S: serde::de::DeserializeOwned, P: AsRef<Path>>(root: P, file: &str) -> Option<S> {
    let file = match read(root.as_ref().join(format!("{}.bin", file))) {
        Ok(file) => file,
        Err(..) => return None,
    };
    postcard::from_bytes::<S>(&file).ok()
}

fn write<S: serde::Serialize, P: AsRef<Path>>(root: P, file: &str, data: S) -> S {
    std::fs::write(
        root.as_ref().join(format!("{}.bin", file)),
        &postcard::to_allocvec(&data).unwrap(),
    )
    .unwrap_or_else(|_| panic!("Cannot make path for {}", file));
    data
}
