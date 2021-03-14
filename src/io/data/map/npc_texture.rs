use firecore_world::serialized::NPCType;
use macroquad::prelude::info;
use macroquad::prelude::warn;

use crate::util::graphics::texture::image_texture;
use crate::util::graphics::Texture;
use crate::world::NpcTextures;
use crate::util::image::byte_image;

lazy_static::lazy_static! {
    static ref BATTLE_SPRITES: dashmap::DashMap<String, Texture> = dashmap::DashMap::new();
}

pub fn load_npc_textures(npc_textures: &mut NpcTextures, npc_types: Vec<NPCType>) { // To - do: make world.bin file with npcs and move chunks.bin and mapsets.bin into one file, with textures included too
    info!("Loading NPC textures...");

    for npc_type in npc_types {
        match byte_image(&npc_type.sprite) {
            Ok(image) => {//match image_texture(&image) {
                // Some(texture) => {
                    let texture = image_texture(&image);
                    macroquad::prelude::debug!("Added NPC texture type: {}", &npc_type.name);
                    if let Some(battle_sprite) = npc_type.battle_sprite {
                        match byte_image(&battle_sprite) {
                            Ok(image) => {
                                BATTLE_SPRITES.insert(npc_type.name.clone(), image_texture(&image));
                            }
                            Err(err) => {
                                warn!("Could not decode NPC type: \"{}\"'s battle texture with error {}", npc_type.name, err);
                            }
                        }
                    }
                    npc_textures.insert(npc_type.name, texture);
                // },
                // None => {
                //     warn!("Could not parse image of three way NPC texture with id {}!", npc_type.name);
                // },
            },
            Err(err) => {
                warn!("Could not parse NPC spritesheet for {} with error {}", npc_type.name, err);
            }
        }
    }
}

// pub fn parse_image(image: Image) -> Option<ThreeWayTexture<StillTextureManager>> {
//     match image.width {
//         48 => idle_npc(image),

//         // Not actually idle npcs, this is temporary
//         144 => idle_npc(image),
//         160 => idle_npc(image),

//         _ => {
//             warn!("Could not parse NPC sprites!");
//             return None;
//         }
//     }

// }

// fn idle_npc(image: Image) -> Option<ThreeWayTexture<StillTextureManager>> {
//     let mut twt = ThreeWayTexture::new();
//     for i in 0..3 {
//         twt.add_texture_manager(StillTextureManager::new(image_texture(&image.sub_image(macroquad::prelude::Rect::new((i << 4) as f32, 0.0, 16.0, 32.0))), false));
//     }
//     return Some(twt);
// }

// pub async fn load_battle_sprites(ids: &[&'static str]) {

//     let base_path = "assets/world/textures/npcs/".to_owned();

//     for id in ids {
//         let path = base_path.clone() + *id + "/battle.png";
//         match macroquad::prelude::load_file(&path).await {
//             Ok(bytes) => {
//                 BATTLE_SPRITES.insert(*id, crate::util::graphics::texture::byte_texture(&bytes));
//             }
//             Err(err) => {
//                 warn!("Could not load battle sprite {} with error {}", id, err);
//             }
//         }
//     }
// }

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