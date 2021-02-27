use macroquad::prelude::Image;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::util::graphics::texture::still_texture_manager::StillTextureManager;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
use crate::util::graphics::texture::image_texture;
use crate::util::graphics::Texture;
use crate::world::NpcTextures;

pub fn load_npc_textures(npc_textures: &mut NpcTextures) {
    info!("Loading NPC textures...");

    //let files = ["idle_up.png", "idle_down.png", "idle_side.png"];
    let dir = "world/textures/npcs";
    for npc_texture_path in crate::io::get_dir(dir) {
        let npc_type =  npc_texture_path.file_name().unwrap().to_string_lossy().into_owned();
        let filepath = std::path::Path::new(dir).join(&npc_type);
        let filepath = filepath.join(npc_type.clone() + ".png");

        match crate::io::get_file(&filepath) {
            Some(file) => {
                match crate::util::image::byte_image(&file) {
                    Ok(image) => match parse_image(image) {
                        Some(twt) => {
                            macroquad::prelude::debug!("Added NPC texture type: {}", &npc_type);
                            npc_textures.insert(npc_type, twt);
                        },
                        None => {
                            warn!("Could not parse image of three way NPC texture with id {}!", &npc_type);
                        },
                    },
                    Err(err) => {
                        warn!("Could not parse NPC spritesheet at {:?} with error {}", filepath, err);
                    }
                }
                
            }
            None => {
                warn!("Could not get texture {} under NPC texture file {:?}", npc_type, filepath);
            },
        }
    }

    info!("Finished loading NPC textures!");
}

pub fn parse_image(image: Image) -> Option<ThreeWayTexture<StillTextureManager>> {
    match image.width {
        48 => idle_npc(image),

        // Not actually idle npcs, this is temporary
        144 => idle_npc(image),
        160 => idle_npc(image),

        _ => {
            warn!("Could not parse NPC sprites!");
            return None;
        }
    }

}

fn idle_npc(image: Image) -> Option<ThreeWayTexture<StillTextureManager>> {
    let mut twt = ThreeWayTexture::new();
    for i in 0..3 {
        twt.add_texture_manager(StillTextureManager::new(image_texture(&image.get_subimage(i * 16, 0, 16, 32)), false));
    }
    return Some(twt);
}

pub fn battle_sprite(id: &str) -> Texture {
    match crate::io::get_file(std::path::PathBuf::from("world/textures/npcs/").join(id).join("battle.png")) {
        Some(file) => {
            return crate::util::graphics::texture::byte_texture(&file);
        }
        None => {
            warn!("Could not find file of battle sprite {}", id);
            return crate::util::graphics::texture::debug_texture();
        }
    }
}