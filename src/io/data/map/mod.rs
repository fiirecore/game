use crate::world::NpcTextures;
use crate::world::TileTextures;
use crate::world::map::chunk::world_chunk_map::WorldChunkMap;
use crate::world::map::set::world_map_set_manager::WorldMapSetManager;

// pub mod map_loader;

pub mod v1;

pub mod npc_texture;

pub fn load_maps() -> (WorldChunkMap, WorldMapSetManager, TileTextures, NpcTextures) {
    let mut wcm = WorldChunkMap::default();
    let mut wmsm = WorldMapSetManager::default();
    let mut tt = TileTextures::new();
    let mut nt = NpcTextures::new();
    v1::load_maps_v1(&mut wcm, &mut wmsm, &mut tt, &mut nt);
    (wcm, wmsm, tt, nt)
}