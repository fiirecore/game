fn main() -> Result<(), Box<dyn std::error::Error>> {

    font_builder::compile("fonts", "build/data/fonts.bin")?;
    #[cfg(feature = "audio")]
    audio_builder::compile("music", "build/data/audio.bin")?;
    dex_builder::compile("pokedex/pokemon", "pokedex/moves", "build/data/dex.bin", cfg!(feature = "audio"))?;
    world_builder::compile("world/maps", "world/textures", "world/npcs", "build/data/world.bin")?;

    embed_resource::compile("build/resources.rc");

    Ok(())
    
}