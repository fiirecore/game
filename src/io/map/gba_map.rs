use log::warn;
use opengl_graphics::{Texture, Filter, TextureSettings};
use std::{fs::File, path::Path};
use std::io::Read;
use std::collections::HashMap;

use image::RgbaImage;

use crate::util::image_util::{open_image, get_subimage};
use crate::util::file_util::asset_as_pathbuf;

pub struct GbaMap {
	
	pub bank: usize,
	pub map: usize,
	pub name: String,
	pub music: u8,
	pub width: u16,
	pub height: u16,
	pub palettes: [usize; 2],
	pub border_blocks: [u16; 4],
	pub tile_map: Vec<u16>,
	pub movement_map: Vec<u8>,
	
}

pub fn get_gba_map<P>(path: P) -> GbaMap where P: AsRef<Path> {
	let path = path.as_ref();

	let mut bytes: Vec<u8> = Vec::new();
	
	match File::open(&path) {
		
		Ok(mut file) => {
			
			file.read_to_end(&mut bytes).expect("Error reading map file!");
			
		}
		
		Err(error) => {
			panic!("Error opening map file {:?}: {}", path, error);
		}
		
	}
	
	let name = get_name_from_id(bytes[44]);
	
	let music = bytes[40];

	let width = bytes[0] as u16; // 0 - 3 reserved
	let height = bytes[4] as u16; // 4 - 7 reserved
	
	let palettes = [bytes[8] as usize, bytes[12] as usize]; // 8 - 11 reserved & 12 - 15 reserved
	
	//let show_name_on_entering = bytes[49];
	
	let mut border_blocks: [u16; 4] = [0; 4];
	
	for x in 0..4 {
		
		let location = 52+x*2;
		
		let tile_num = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
		
		border_blocks[x] = tile_num;
		
	}
	
	let mut tile_map: Vec<u16> = Vec::new();
	let mut movement_map: Vec<u8> = Vec::new();
	
	let size = width as usize * height as usize;
	
	for x in 0..size {
		
		let location = 60 + x * 2;
		
		let tile_num = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
		
		let move_num = (bytes[location+1]/4) as u8;
		
		tile_map.push(tile_num);
		movement_map.push(move_num);
		
	}

	GbaMap {
		
		bank: 0,//bank,
		map: 0, //map,
		name: String::from(name),
		music: music,
		width: width,
		height: height,
		palettes: palettes,
		border_blocks: border_blocks,
		tile_map: tile_map,
		movement_map: movement_map,
//		spawnpoint: _spawnpoint,
		
	}
	
}

pub fn fix_tiles(gba_map: &mut GbaMap, palette_sizes: &Vec<u16>) {

	let offset = get_offset(gba_map, palette_sizes);	

	fix_tiles2(&mut gba_map.tile_map, offset, palette_sizes[0]);	

	for index in 0..gba_map.border_blocks.len() {
		if gba_map.border_blocks[index] > palette_sizes[0] {
			gba_map.border_blocks[index] += offset;
		}
	}

}

pub fn fix_tiles2(tiles: &mut Vec<u16>, offset: u16, first_palette_size: u16) {
	for index in 0..tiles.len() {
		if tiles[index] > first_palette_size {
			tiles[index] += offset;
		}
	}
}

pub fn get_offset(gba_map: &GbaMap, palette_sizes: &Vec<u16>) -> u16 {
	let mut offset = 0;
	for x in 1..gba_map.palettes[1] {
		offset += palette_sizes[x];
	}
	return offset;
}

fn get_name_from_id(id: u8) -> &'static str {
	match id {
		0x7D => return "Route 25",

		_ => return "Unknown Map ID",
	}
}

// Map conversion utility

pub fn fill_palette_map(bottom_sheets: &mut HashMap<u8, RgbaImage>, top_sheets: &mut HashMap<u8, RgbaImage>, world_id: &String, to_index: u8) {
    for index in 0..to_index+1 {
        let mut bottom: String = String::from("worlds/");
        bottom.push_str(world_id);
        bottom.push_str("/textures/Palette");
        bottom.push_str(&index.to_string());
		let mut top = bottom.clone();
		bottom.push_str("B.png");
		top.push_str("T.png");
		if let Ok(img) = open_image(asset_as_pathbuf(bottom)) {
			bottom_sheets.insert(index, img);
		}
		if let Ok(img) = open_image(asset_as_pathbuf(top)) {
			top_sheets.insert(index, img);
		}
		
    }
}

pub fn get_texture(sheets: &HashMap<u8, RgbaImage>, palette_sizes: &Vec<u16>, tile_id: u16) -> Texture {
	let img: RgbaImage;
	
	let mut count: u16 = palette_sizes[0];
	let mut index: u8 = 0;

	while tile_id >= count {
		index += 1;
		count += palette_sizes[index as usize];
	}

	match sheets.get(&index) {
		Some(sheet) => {
			img = get_subimage(sheet, (tile_id - (count - palette_sizes[index as usize])) as usize);
			return Texture::from_image(&img, &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest));
		}
		None => {
			let mut string = String::from("Could not get spritesheet at index ");
			string.push_str(index.to_string().as_str());
			warn!("{}", string);
			println!("{:?}", sheets.keys());
			return Texture::from_path(asset_as_pathbuf("debug/missing_texture.png"), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).expect("Could not find debug texture");
		}
	}
    
}

