
use ahash::AHashMap as HashMap;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::manager::WorldMapSetManager;
use macroquad::prelude::Image;
use macroquad::prelude::info;
use super::gba_map;

pub async fn load_maps_v2(tile_textures: &mut crate::world::TileTextures, npc_textures: &mut crate::world::NpcTextures) -> (WorldChunkMap, WorldMapSetManager) {

    info!("Loading maps...");

    let world: firecore_world::serialized::SerializedWorld = bincode::deserialize(
        &macroquad::prelude::load_file("assets/world.bin").await.unwrap()
        // include_bytes!("../../../../../assets/world.bin")
    ).unwrap();

    info!("Loaded world file.");
    
    let chunk_map: WorldChunkMap = world.chunks;
    let map_set_manager: WorldMapSetManager = world.map_sets;

    let images: Vec<(u8, Image)> = world.palettes.into_iter().map(|palette|
        (palette.id, Image::from_file_with_format(&palette.bottom, Some(image::ImageFormat::Png)))
    ).collect();
    
    let mut bottom_sheets = HashMap::new();
    let mut palette_sizes = HashMap::new();
    for (id, image) in images {
        palette_sizes.insert(id, (image.width >> 4) * (image.height >> 4));
        bottom_sheets.insert(id, image);
    }

    info!("Finished loading maps!");

    info!("Loading textures...");
    for tile_id in chunk_map.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }
    for tile_id in map_set_manager.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }

    super::npc_texture::load_npc_textures(npc_textures, world.npc_types);
    info!("Finished loading textures!");

    (chunk_map, map_set_manager)

}