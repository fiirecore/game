// extern crate map_compressor;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    map_compressor::with_dirs("world/maps", "embed/world/textures/tiles", "assets")?;

    #[cfg(all(windows, not(debug_assertions)))] {
        let mut res = winres::WindowsResource::new();
        res.set_icon("build/icon.ico");
        res.compile()?;
    }

    Ok(())
    
}