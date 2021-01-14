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
	//pub name: String,
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
	
	//let name = get_name_from_id(bytes[44]);
	
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
		//name: String::from(name),
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
	
	for index in 0..gba_map.tile_map.len() {
		if gba_map.tile_map[index] > palette_sizes[0] {
			gba_map.tile_map[index] += offset;
		}
	}

	for index in 0..gba_map.border_blocks.len() {
		if gba_map.border_blocks[index] > palette_sizes[0] {
			gba_map.border_blocks[index] += offset;
		}
	}

	if gba_map.palettes[0] > 0 {
		let mut offset12: u16 = 0;

		for x in 0..gba_map.palettes[0] {
			offset12 += palette_sizes[x];
		}

		for index in 0..gba_map.tile_map.len() {
			if gba_map.tile_map[index] < palette_sizes[0] {
				gba_map.tile_map[index] += offset12;
			}
		}

		for index in 0..gba_map.border_blocks.len() {
			if gba_map.border_blocks[index] < palette_sizes[0] {
				gba_map.border_blocks[index] += offset12;
			}
		}
	}

}
pub fn get_offset(gba_map: &GbaMap, palette_sizes: &Vec<u16>) -> u16 { // To - do: change to recursive function
	let mut offset = 0;
	if gba_map.palettes[1] >= palette_sizes.len() {
		warn!("Not enough palettes to support gba map textures. Need palette #{}", gba_map.palettes[1]);
		return 0;
	}
	for x in 1..gba_map.palettes[1] {
		offset += palette_sizes[x];
	}
	return offset;
}

/*
fn get_name_from_id(id: u8) -> &'static str {
	match id {
		0x7D => return "Route 25",

		_ => return "Unknown Map ID",
	}
}
*/

// Map conversion utility

pub fn fill_palette_map(bottom_sheets: &mut HashMap<u8, RgbaImage>, top_sheets: &mut HashMap<u8, RgbaImage>, world_id: &String) -> Vec<u16> {
	let mut sizes = Vec::new();
	let mut count = 0;
	loop {

		let file = asset_as_pathbuf("worlds").join(world_id).join("textures");

		let mut bottom_filename = String::from("Palette");
		bottom_filename.push_str(&count.to_string());

		let mut top_filename = bottom_filename.clone();

		bottom_filename.push_str("B.png");
		top_filename.push_str("T.png");	
		
		let bottom_file = file.join(bottom_filename);

		if !bottom_file.exists() {
			break;
		}
		if let Ok(img) = open_image(bottom_file) {
			sizes.push(((img.width() >> 4) * (img.height() >> 4)) as u16);
			bottom_sheets.insert(count, img);
		}
		if let Ok(img) = open_image(file.join(top_filename)) {
			top_sheets.insert(count, img);
		}
		count+=1;
	}
	sizes
}

pub fn get_texture(sheets: &HashMap<u8, RgbaImage>, palette_sizes: &Vec<u16>, tile_id: u16) -> Texture {
	let img: RgbaImage;
	
	let mut count: u16 = palette_sizes[0];
	let mut index: u8 = 0;

	while tile_id >= count {
		index += 1;
		if index >= (palette_sizes.len() as u8) {
			warn!("Tile ID {} exceeds palette texture count!", tile_id);
			break;
		}
		count += palette_sizes[index as usize];
	}

	match sheets.get(&index) {
		Some(sheet) => {
			img = get_subimage(sheet, (tile_id - (count - palette_sizes[index as usize])) as usize);
			return Texture::from_image(&img, &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest));
		}
		None => {
			warn!("Could not get texture for tile ID {}", &tile_id);
			return Texture::from_path(asset_as_pathbuf("debug/missing_texture.png"), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).expect("Could not find debug texture");
		}
	}
    
}

