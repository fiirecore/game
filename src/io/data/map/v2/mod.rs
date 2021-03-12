
use ahash::AHashMap as HashMap;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::manager::WorldMapSetManager;
use macroquad::prelude::info;
use super::gba_map;

pub async fn load_maps_v2(tile_textures: &mut crate::world::TileTextures, npc_textures: &mut crate::world::NpcTextures) -> (WorldChunkMap, WorldMapSetManager) {
    let mut bottom_sheets: HashMap<u8, macroquad::prelude::Image> = HashMap::new();
    let palette_sizes = gba_map::fill_palette_map(&mut bottom_sheets).await;

    info!("Loading maps...");
    
    let chunk_map: WorldChunkMap = bincode::deserialize(&macroquad::prelude::load_file("assets/chunks.bin").await.unwrap()).unwrap();
    let map_set_manager: WorldMapSetManager = bincode::deserialize(&macroquad::prelude::load_file("assets/mapsets.bin").await.unwrap()).unwrap();

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

    super::npc_texture::load_npc_textures(npc_textures).await;
    info!("Finished loading textures!");

    (chunk_map, map_set_manager)

}