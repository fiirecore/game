fn main() -> Result<(), Box<dyn std::error::Error>> {

    font_builder::build_font("fonts", "assets/fonts.bin")?;
    #[cfg(feature = "audio")]
    audio_builder::compile("music", "assets/audio.bin")?;
    dex_builder::compile("pokedex/pokemon", "pokedex/moves", "assets/dex.bin", cfg!(feature = "audio"))?;
    world_builder::compile("world/maps", "world/textures", "world/npcs", "assets/world.bin")?;

    embed_resource::compile("build/resources.rc");

    Ok(())
    
}