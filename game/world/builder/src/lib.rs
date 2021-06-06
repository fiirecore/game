extern crate firecore_world_lib as worldlib;
extern crate firecore_util as util;

use std::io::Write;
use std::path::Path;

use firecore_pokedex_game::serialize::SerializedDex;
use worldlib::map::manager::WorldMapManager;
use worldlib::map::warp::WarpEntry;

pub mod world;
pub mod gba_map;

mod dex;

pub fn compile<P: AsRef<Path>>(dex: SerializedDex, root_path: P, output_file: P) {

    dex::setup(dex);

    println!("Started loading maps and tile textures...");
    let (manager, mut textures) = world::map::load_maps(root_path.as_ref());
    println!("Finished loading maps and tile textures.");

    println!("Verifying palettes, maps, warps...");
    verify_palettes(&manager, &mut textures);
    verify_warps(&manager);
    verify_connections(&manager);

    println!("Loading Npc types...");
    let npc_types = world::npc::npc_type::load_npc_types(root_path.as_ref());

    let output_file = output_file.as_ref();

    if let Some(parent) = output_file.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| panic!("Could not create directories for output file with error {}", err));
        }
    }
    
    let mut file = std::fs::File::create(output_file).unwrap_or_else(|err| panic!("Could not create output file at {:?} with error {}", output_file, err));

    let data = firecore_world_lib::serialized::SerializedWorld {
        manager,
        npc_types,
        textures,
    };

    println!("Saving data...");
    let bytes = firecore_dependencies::ser::serialize(&data).unwrap_or_else(|err| panic!("Could not serialize output file with error {}", err));
    let bytes = file.write(&bytes).unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
    println!("Wrote {} bytes to world file!", bytes);

}

fn verify_palettes(manager: &WorldMapManager, textures: &mut worldlib::serialized::SerializedTextures) {
    let keys = textures.palettes.keys().copied().collect::<Vec<worldlib::map::PaletteId>>();
    let mut palettes = Vec::new();
    for map in manager.maps.values() {
        for palette in map.palettes.iter() {
            if !palettes.contains(palette) {
                palettes.push(*palette);
            }
        }
    }
    for palette in &keys {
        if !palettes.contains(palette) {
            eprintln!("Palette #{} is not used!", palette);
            textures.palettes.remove(palette);
        }
    }
    for palette in palettes {
        if !keys.contains(&palette) {
            panic!("Palette #{} is missing!", palette);
        }
    }
}

fn verify_warps(manager: &WorldMapManager) {
    let mut errors: u32 = 0;
    for map in manager.maps.values() {
        for warp in map.warps.values() {
            errors += verify_warp(warp, &map.name, &manager);
        }
    }
    if errors != 0 {
        panic!("Found {} errors in warp files.", errors);
    }
}

fn verify_warp(warp: &WarpEntry, map_name: &str, manager: &WorldMapManager) -> u32 {
    let mut errors: u32 = 0;
    if !manager.maps.contains_key(&warp.destination.location) {
        eprintln!("Map {} contains a warp to non-existent map {}", map_name, warp.destination.location.index);
        errors += 1;
    }
    errors 
}

fn verify_connections(manager: &WorldMapManager) {
    let mut errors: u32 = 0;
    for map in manager.maps.values() {
        if let Some(chunk) = &map.chunk {
            for connection in &chunk.connections {
                if !manager.maps.contains_key(connection) {
                    eprintln!("Could not get connection \"{}\" for chunk {}", connection, map.name);
                    errors += 1;
                }
            }
            
        }
    }
    if errors != 0 {
        panic!("Found {} errors in chunk connections.", errors)
    }
}