use firecore_world::serialized::SerializedNPCType;
use macroquad::prelude::info;
use macroquad::prelude::warn;

use crate::util::graphics::texture::image_texture;
use crate::util::graphics::Texture;
use crate::world::NpcTextures;
use crate::util::image::byte_image;

lazy_static::lazy_static! {
    static ref BATTLE_SPRITES: dashmap::DashMap<String, Texture> = dashmap::DashMap::new();
}

pub fn load_npc_textures(npc_textures: &mut NpcTextures, npc_types: Vec<SerializedNPCType>) { // To - do: make world.bin file with npcs and move chunks.bin and mapsets.bin into one file, with textures included too
    info!("Loading NPC textures...");

    for npc_type in npc_types {
        match byte_image(&npc_type.sprite) {
            Ok(image) => {//match image_texture(&image) {
                // Some(texture) => {
                    let texture = image_texture(&image);
                    macroquad::prelude::debug!("Added NPC texture type: {}", &npc_type.identifier);
                    if let Some(battle_sprite) = npc_type.battle_sprite {
                        match byte_image(&battle_sprite) {
                            Ok(image) => {
                                BATTLE_SPRITES.insert(npc_type.identifier.clone(), image_texture(&image));
                            }
                            Err(err) => {
                                warn!("Could not decode NPC type: \"{}\"'s battle texture with error {}", npc_type.identifier, err);
                            }
                        }
                    }
                    crate::world::npc::NPC_TYPES.insert(npc_type.identifier.clone(), npc_type.data);
                    npc_textures.insert(npc_type.identifier, texture);
                // },
                // None => {
                //     warn!("Could not parse image of three way NPC texture with id {}!", npc_type.name);
                // },
            },
            Err(err) => {
                warn!("Could not parse NPC spritesheet for {} with error {}", npc_type.identifier, err);
            }
        }
    }
}

pub fn battle_sprite(id: &str) -> Texture {
    match BATTLE_SPRITES.get(id) {
        Some(texture) => {
            *texture
        }
        None => {
            crate::util::graphics::texture::debug_texture()
        }
    }
}