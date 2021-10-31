fn main() {
    println!("cargo:rerun-if-changed=assets");

    firecore_font_builder::compile("assets/game/fonts", "build/data/fonts.bin");
    #[cfg(feature = "audio")]
    firecore_audio_builder::compile("assets/game/music", "build/data/audio.bin");
    let dex = firecore_pokedex_builder::compile(
        "assets/game/pokedex/pokemon",
        "assets/game/pokedex/moves",
        "assets/game/pokedex/items",
        // ,
        // Some("build/data/dex.bin"),
        // cfg!(feature = "audio"),
    );

    std::fs::write("build/data/dex.bin", bincode::serialize(&dex).unwrap()).unwrap();

    let dex_engine = firecore_pokedex_engine_builder::compile("assets/game/pokedex/client/pokemon", "assets/game/pokedex/client/items", "assets/game/pokedex/client/trainers");

    std::fs::write("build/data/dex_engine.bin", bincode::serialize(&dex_engine).unwrap()).unwrap();

    firecore_world_builder::compile("assets/game/world", "build/data/world.bin");

    let battle = std::path::Path::new("assets/game/pokedex/battle");

    let battle = firecore_battle_builder::compile(battle, &battle.join("scripts"));

    std::fs::write("build/data/battle.bin", bincode::serialize(&battle).unwrap()).unwrap();

    #[cfg(windows)]
    // embed_resource::compile("build/resources.rc");
    winres::WindowsResource::new()
        .set_icon("build/icon.ico")
        .compile()
        .unwrap();
}
