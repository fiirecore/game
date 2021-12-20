extern crate firecore_world as worldlib;

use worldlib::{
    map::{chunk::Connection, manager::Maps, warp::WarpEntry, PaletteId},
    serialized::{SerializedTextures, SerializedWorld},
};

pub mod gba_map;
pub mod world;

pub fn compile(path: impl AsRef<std::path::Path>) -> SerializedWorld {
    println!("Started loading maps and tile textures...");
    let (maps, mut textures) = world::map::load_world(path.as_ref());
    println!("Finished loading maps and tile textures.");

    println!("Verifying palettes, maps, warps...");
    verify_palettes(&maps, &mut textures);
    verify_warps(&maps);
    verify_connections(&maps);

    println!("Loading Npc types...");
    let npc_types = world::npc::npc_type::load_npc_types(path.as_ref());

    let data = SerializedWorld {
        maps,
        npc_types,
        textures,
        // map_gui_locs,
    };

    data
}

fn verify_palettes(maps: &Maps, textures: &mut SerializedTextures) {
    let keys = textures
        .palettes
        .keys()
        .copied()
        .collect::<Vec<PaletteId>>();
    let mut palettes = Vec::new();
    for map in maps.values() {
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

fn verify_warps(maps: &Maps) {
    let mut errors: u32 = 0;
    for map in maps.values() {
        for warp in map.warps.values() {
            errors += verify_warp(warp, &map.name, maps);
        }
    }
    if errors != 0 {
        panic!("Found {} errors in warp files.", errors);
    }
}

fn verify_warp(warp: &WarpEntry, map_name: &str, maps: &Maps) -> u32 {
    let mut errors: u32 = 0;
    if !maps.contains_key(&warp.destination.location) {
        eprintln!(
            "Map {} contains a warp to non-existent map {}",
            map_name, warp.destination.location
        );
        errors += 1;
    }
    errors
}

fn verify_connections(maps: &Maps) {
    let mut errors: u32 = 0;
    for map in maps.values() {
        if let Some(chunk) = &map.chunk {
            for Connection(location, ..) in chunk.connections.values() {
                if !maps.contains_key(location) {
                    eprintln!(
                        "Could not get connection \"{}\" for chunk {}",
                        location, map.name
                    );
                    errors += 1;
                }
            }
        }
    }
    if errors != 0 {
        panic!("Found {} errors in chunk connections.", errors);
    }
}
