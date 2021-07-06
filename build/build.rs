fn main() {
    println!("cargo:rerun-if-changed=assets");

    firecore_font_builder::compile("assets/game/fonts", "build/data/fonts.bin");
    #[cfg(feature = "audio")]
    firecore_audio_builder::compile("assets/game/music", "build/data/audio.bin");
    let dex = firecore_pokedex_builder::compile(
        "assets/game/pokedex/pokemon",
        "assets/game/pokedex/moves",
        "assets/game/pokedex/items",
        "assets/game/pokedex/trainers",
        Some("build/data/dex.bin"),
        cfg!(feature = "audio"),
    );
    firecore_world_builder::compile(dex, "assets/game/world", "build/data/world.bin");

    // embed_resource::compile("build/resources.rc");
    winres::WindowsResource::new()
        .set_icon("build/icon.ico")
        .compile()
        .unwrap();
}
