fn main() {

    println!("cargo:rerun-if-changed=assets");

    font_builder::compile("assets/fonts", "build/data/fonts.bin");
    #[cfg(feature = "audio")]
    audio_builder::compile("assets/music", "build/data/audio.bin");
    dex_builder::compile("assets/pokedex/pokemon", "assets/pokedex/moves", "assets/pokedex/items", "build/data/dex.bin", cfg!(feature = "audio"));
    world_builder::compile("assets/world/maps", "assets/world/textures", "assets/world/npcs", "build/data/world.bin");

    embed_resource::compile("build/resources.rc");
    
}