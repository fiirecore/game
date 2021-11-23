fn write<S: serde::Serialize>(file: &str, data: &S) {
    std::fs::write(format!("build/data/{}.bin", file), bincode::serialize(data).unwrap()).unwrap()
}

fn main() {
    println!("cargo:rerun-if-changed=assets");

    write("fonts", &firecore_font_builder::compile("assets/game/fonts"));

    #[cfg(feature = "audio")]
    write("audio", &firecore_audio_builder::compile("assets/music"));

    let dex = firecore_pokedex_builder::compile(
        "assets/game/pokedex/pokemon",
        "assets/game/pokedex/moves",
        "assets/game/pokedex/items",
    );

    write("dex", &dex);

    let dex_engine = firecore_pokedex_engine_builder::compile("assets/game/pokedex/client/pokemon", "assets/game/pokedex/client/items", "assets/game/pokedex/client/trainers");

    write("dex_engine", &dex_engine);

    let world = firecore_world_builder::compile("assets/world");

    write("world", &world);

    let battle = std::path::Path::new("assets/game/pokedex/battle");

    let battle = firecore_battle_builder::compile(battle, &battle.join("scripts"));

    write("battle", &battle);

    // #[cfg(windows)]
    // // embed_resource::compile("build/resources.rc");
    // winres::WindowsResource::new()
    //     .set_icon("build/icon.ico")
    //     .compile()
    //     .unwrap();
}
