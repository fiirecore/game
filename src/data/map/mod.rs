use ahash::AHashMap as HashMap;
use firecore_world::map::manager::WorldMapManager;
use macroquad::prelude::Image;
use macroquad::prelude::info;

use crate::util::{TILE_SIZE, graphics::Texture};


pub mod npc_texture;

pub async fn load_maps(tile_textures: &mut crate::world::TileTextures, npc_textures: &mut crate::world::NpcTextures) -> WorldMapManager {

    info!("Loading maps...");

    let world: firecore_world::serialized::SerializedWorld = bincode::deserialize(
        &macroquad::prelude::load_file("assets/world.bin").await.unwrap()
        // include_bytes!("../../../assets/world.bin")
    ).unwrap();

    info!("Loaded world file.");

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
    for tile_id in world.manager.chunk_map.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }
    for tile_id in world.manager.map_set_manager.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }

    npc_texture::load_npc_textures(npc_textures, world.npc_types);
    info!("Finished loading textures!");

    world.manager

}

pub fn get_texture(sheets: &HashMap<u8, Image>, palette_sizes: &HashMap<u8, u16>, tile_id: u16) -> Texture {
	
	let mut count: u16 = *palette_sizes.get(&0).unwrap();
	let mut index: u8 = 0;

	while tile_id >= count {
		index += 1;
		if index >= (palette_sizes.len() as u8) {
			macroquad::prelude::warn!("Tile ID {} exceeds palette texture count!", tile_id);
			break;
		}
		count += *palette_sizes.get(&index).unwrap();
	}

	match sheets.get(&index) {
		Some(sheet) => {
			let id = (tile_id - (count - *palette_sizes.get(&index).unwrap())) as usize;
			crate::util::graphics::texture::image_texture(
				&sheet.sub_image(
					macroquad::prelude::Rect::new(
						((id % (sheet.width() / TILE_SIZE as usize)) * TILE_SIZE as usize) as f32, 
						((id / (sheet.width() / TILE_SIZE as usize)) * TILE_SIZE as usize) as f32,
						TILE_SIZE as f32,
						TILE_SIZE as f32,
					)
				)
			)
		}
		None => {
			macroquad::prelude::debug!("Could not get texture for tile ID {}", &tile_id);
			crate::util::graphics::texture::debug_texture()
		}
	}
    
}