fn write<S: serde::Serialize>(file: &str, data: &S) {

    let dest = firecore_storage::directory(false, PUBLISHER, APPLICATION).unwrap().join("assets");
    if !dest.exists() {
        std::fs::create_dir_all(&dest).unwrap();
    }

    std::fs::write(
        dest.join(format!("{}.bin", file)),
        firecore_storage::to_bytes(data).unwrap(),
    )
    .unwrap()
}

const PUBLISHER: Option<&str> = Some("fiirecore");
const APPLICATION: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("cargo:rerun-if-changed=assets");

    write(
        "fonts",
        &firecore_font_builder::compile("assets/game/fonts"),
    );

    #[cfg(feature = "audio")]
    write("audio", &firecore_audio_builder::compile("assets/music"));

    let dex = firecore_pokedex_builder::compile(
        "assets/game/pokedex/pokemon",
        "assets/game/pokedex/moves",
        "assets/game/pokedex/items",
    );

    write("dex", &dex);

    let dex_engine = firecore_pokedex_engine_builder::compile(
        "assets/game/pokedex/client/pokemon",
        "assets/game/pokedex/client/items",
        "assets/game/pokedex/client/trainers",
    );

    write("dex_engine", &dex_engine);

    let world = firecore_world_builder::compile("assets/world");

    write("world", &world);

    let battle = std::path::Path::new("assets/game/pokedex/battle");

    let battle = firecore_battle_builder::compile(battle, &battle.join("scripts"));

    write("battle", &battle);

    // let ext = Some(std::ffi::OsString::from("bin"));

    // let ext = ext.as_deref();

    #[cfg(windows)]
    embed_resource::compile("build/resource.rc");
}
