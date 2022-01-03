pub extern crate firecore_world as world;

use world::{
    map::{chunk::Connection, manager::{Maps, WorldMapData}, warp::WarpEntry, PaletteId},
    serialized::{SerializedTextures, SerializedWorld},
};

pub mod builder;

pub mod bin;
// mod gba_map;

pub fn compile(path: impl AsRef<std::path::Path>) -> SerializedWorld {
    println!("Started loading maps and tile textures...");
    let (maps, mut textures) = builder::map::load_world(path.as_ref());
    println!("Finished loading maps and tile textures.");

    let builder::BuilderWorldData { tiles, wild, spawn } = {
        let path = path.as_ref().join("data.ron");
        ron::from_str::<builder::BuilderWorldData>(&std::fs::read_to_string(&path).unwrap_or_else(
            |err| {
                panic!(
                    "Could not read world data file at {:?} with error {}",
                    path, err
                )
            },
        ))
        .unwrap_or_else(|err| {
            panic!(
                "Could not deserialize world data file at {:?} with error {}",
                path, err
            )
        })
    };

    println!("Verifying palettes, maps, warps...");
    verify_palettes(&maps, &mut textures);
    verify_warps(&maps);
    verify_connections(&maps);

    println!("Loading Npc types...");
    let (npcs, npc_textures) = builder::npc::group::load_npc_types(path.as_ref());

    textures.npcs = npc_textures;

    let data = SerializedWorld {
        data: WorldMapData {
            maps,
            tiles,
            npcs,
            wild,
            spawn,
        },
        textures,
    };

    data
}

fn filename(path: &std::path::Path) -> String {
    path.file_stem()
        .map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(|| panic!("Could not read the file stem of file at {:?}", path))
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
        for warp in map.warps.iter() {
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
            for Connection(location, ..) in chunk.connections.values().flatten() {
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
