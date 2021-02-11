pub mod map_serializable;

pub mod map_loader;
pub mod chunk_map_loader;
pub mod map_set_loader;

pub mod warp_loader;
pub mod npc {
    pub mod npc_loader;
    pub mod npc_sprite;
}
pub mod wild_entry_loader;

pub mod gba_map;
pub mod json_map {
    pub mod v1;
}