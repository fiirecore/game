use macroquad::prelude::Image;
use macroquad::prelude::warn;
use crate::entity::texture::still_texture_manager::StillTextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::util::graphics::texture::image_texture;

pub fn parse_image(image: Image) -> Option<ThreeWayTexture> {
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

fn idle_npc(image: Image) -> Option<ThreeWayTexture> {
    let mut twt = ThreeWayTexture::new();
    for i in 0..3 {
        twt.add_texture_manager(Box::new(StillTextureManager::new(image_texture(&image.get_subimage(i * 16, 0, 16, 32)), false)));
    }
    return Some(twt);
}