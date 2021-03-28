use firecore_world::serialized::SerializedNPCType;
use macroquad::prelude::info;

use crate::battle::manager::BattleManager;
use crate::util::graphics::byte_texture;
use crate::world::NPCTypes;
use crate::world::NpcTextures;

pub fn load_npc_textures(battle_manager: &mut BattleManager, npc_textures: &mut NpcTextures, npc_types: &mut NPCTypes, serialized_npc_types: Vec<SerializedNPCType>) {
    info!("Loading NPC textures...");

    for npc_type in serialized_npc_types {
        let texture = byte_texture(&npc_type.sprite);
        if let Some(battle_sprite) = npc_type.battle_sprite {
            battle_manager.trainer_sprites.insert(npc_type.identifier.clone(), byte_texture(&battle_sprite));
        }
        npc_types.insert(npc_type.identifier.clone(), npc_type.data);
        npc_textures.insert(npc_type.identifier, texture);
    }
}