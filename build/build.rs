fn main() -> Result<(), Box<dyn std::error::Error>> {

    font_builder::build_font("fonts", "assets/fonts.bin")?;
    audio_builder::compile("music", "assets/audio.bin")?;
    dex_builder::build_dex("pokedex/entries", "pokedex/moves", "pokedex/textures", "assets/dex.bin")?;
    world_builder::compile("world/maps", "world/textures", "world/npcs", "assets/world.bin")?;

    embed_resource::compile("build/resources.rc");

    Ok(())
    
}