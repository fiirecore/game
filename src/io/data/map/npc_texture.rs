use macroquad::prelude::Image;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::util::graphics::texture::still_texture_manager::StillTextureManager;
use crate::util::graphics::texture::three_way_texture::ThreeWayTexture;
use crate::util::graphics::texture::image_texture;
use crate::util::graphics::Texture;
use crate::world::NpcTextures;

#[deprecated]
pub async fn load_npc_textures(npc_textures: &mut NpcTextures) {
    info!("Loading NPC textures...");

    //let files = ["idle_up.png", "idle_down.png", "idle_side.png"];
    let dir = "assets/world/textures/npcs";
    for npc_texture_path in 
    &[
        "brock",
        "bug_catcher",
        "camper",
        "fat_man",
        "lass",
        "misty",
        "mom",
        "prof_oak",
        "woman_1",
        "youngster"
    ] {
        let npc_texture_path = *npc_texture_path;
        // let npc_type =  npc_texture_path.file_name().unwrap().to_string_lossy().into_owned();
        let npc_type = npc_texture_path.to_owned();
        // let filepath = std::path::Path::new(dir).join(&npc_type);
        // let filepath = filepath.join(npc_type.clone() + ".png");
        let filepath = format!("{}/{}/{}.png", dir, npc_texture_path, npc_texture_path);

        match macroquad::prelude::load_file(&filepath).await {// crate::io::get_file(&filepath) {
            Ok(file) => {
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
            Err(err) => {
                warn!("Could not get texture {} under NPC texture file {:?} with error {}", npc_type, filepath, err);
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
        twt.add_texture_manager(StillTextureManager::new(image_texture(&image.sub_image(macroquad::prelude::Rect::new((i << 4) as f32, 0.0, 16.0, 32.0))), false));
    }
    return Some(twt);
}

pub fn battle_sprite(id: &str) -> Texture {
    match crate::util::file::noasync::read_noasync(std::path::PathBuf::from("assets/world/textures/npcs/").join(id).join("battle.png")) {
        Some(file) => {
            return crate::util::graphics::texture::byte_texture(&file);
        }
        None => {
            warn!("Could not find file of battle sprite {}", id);
            return crate::util::graphics::texture::debug_texture();
        }
    }
}