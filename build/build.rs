fn main() -> Result<(), Box<dyn std::error::Error>> {

    firecore_font_builder::build_font("fonts", "assets/fonts.bin")?;
    dex_builder::build_dex("pokedex/entries", "pokedex/moves", "pokedex/textures", "assets/dex.bin")?;
    world_builder::with_dirs("world/maps", "world/textures/tiles", "world/textures/npcs", "assets/world.bin")?;

    #[cfg(all(windows, not(debug_assertions)))] {
        let mut res = winres::WindowsResource::new();
        res.set_icon("build/icon.ico");
        res.compile()?;
    }

    Ok(())
    
}