use std::fs::File;
use std::io::Write;

use firecore_pokedex::moves::PokemonMove;

// use zip::write::FileOptions;

// extern crate map_compressor;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // zip_dir("pokedex/moves", "assets/moves.zip")?;
    save_dex("pokedex/entries", "pokedex/moves", "assets/dex.bin")?;
    map_compressor::with_dirs("world/maps", "assets/world/textures/tiles", "assets")?;

    #[cfg(all(windows, not(debug_assertions)))] {
        let mut res = winres::WindowsResource::new();
        res.set_icon("build/icon.ico");
        res.compile()?;
    }

    Ok(())
    
}

fn save_dex(pokemon_dir: &str, move_dir: &str, save_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create(save_file)?;

    let mut pokemon = Vec::new();
    let mut moves = Vec::new();

    if let Ok(readdir) = std::fs::read_dir(pokemon_dir) {
        for entry in readdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(data) = std::fs::read_to_string(path) {
                    if let Ok(pokemon_entry) = toml::from_str(&data) {
                        pokemon.push(pokemon_entry);
                    }
                }
            }
        }
    }

    if let Ok(readdir) = std::fs::read_dir(move_dir) {
        for entry in readdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(data) = std::fs::read_to_string(path) {
                    if let Ok(pokemon_move) = toml::from_str(&data) {
                        moves.push(pokemon_move);
                    }
                }
            }
        }
    }

    let data = DexSerialized {
        pokemon,
        moves
    };

    let data = bincode::serialize(&data)?;

    file.write_all(&data)?;

    Ok(())

}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DexSerialized {

	pub pokemon: Vec<firecore_pokedex::pokemon::Pokemon>,
	pub moves: Vec<PokemonMove>,

}

// fn zip_dir(dir: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {

//     let file = File::create(output_file)?;

//     let walkdir = walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok());

//     let mut zip = zip::ZipWriter::new(file);

//     let options = FileOptions::default()
//         .compression_method(zip::CompressionMethod::Bzip2)
//         .unix_permissions(0o755);

//     let mut buffer = Vec::new();

//     for entry in walkdir {
//         let path = entry.path();
//         let name = path.file_name().unwrap().to_string_lossy().to_string();
//         zip.start_file(name, options)?;
//         let mut f = File::open(path)?;
//         f.read_to_end(&mut buffer)?;
//         zip.write_all(&*buffer)?;
//         buffer.clear();
//     }

//     zip.finish()?;
//     Ok(())

// }