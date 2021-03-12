fn main() -> Result<(), Box<dyn std::error::Error>> {

    dex_builder::build_dex("pokedex/entries", "pokedex/moves", "assets/dex.bin")?;
    map_compressor::with_dirs("world/maps", "assets/world/textures/tiles", "assets")?;

    // #[cfg(all(windows, not(debug_assertions)))] {
    //     let mut res = winres::WindowsResource::new();
    //     res.set_icon("build/icon.ico");
    //     res.compile()?;
    // }

    Ok(())
    
}