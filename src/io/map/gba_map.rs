use macroquad::prelude::Image;
use macroquad::prelude::debug;
use macroquad::prelude::warn;
use ahash::AHashMap;
use crate::util::texture::Texture;

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

pub fn get_gba_map(file: include_dir::File) -> GbaMap  {

	let bytes = file.contents();
			
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

// Map conversion utility

pub fn fill_palette_map(bottom_sheets: &mut AHashMap<u8, Image>/*, top_sheets: &mut HashMap<u8, RgbaImage>*/) -> Vec<u16> {
	let texture_dir = crate::io::ASSET_DIR.get_dir("world/textures").expect("Could not get texture directory");
	let mut sizes = Vec::new();
	for file in texture_dir.files() {
		let filename = &*file.path().file_name().unwrap().to_string_lossy();
		if filename.starts_with("P") {
			if filename.ends_with("B.png") {
				match &filename[7..filename.len()-5].parse::<u8>() {
				    Ok(index) => {
						let img = crate::util::image::open_image_bytes(file.contents());
						sizes.push(((img.width() >> 4) * (img.height() >> 4)) as u16);
						bottom_sheets.insert(*index, img);
					}
				    Err(err) => {
						warn!("Could not parse tile palette named {} with error {}", filename, err);
					}
				}
			}
		}
	}
	sizes
}

pub fn get_texture(sheets: &AHashMap<u8, Image>, palette_sizes: &Vec<u16>, tile_id: u16) -> Texture {
	
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
			return crate::util::texture::image_texture(&crate::util::image::get_subimage(sheet, (tile_id - (count - palette_sizes[index as usize])) as usize));
		}
		None => {
			debug!("Could not get texture for tile ID {}", &tile_id);
			return crate::util::texture::debug_texture();
		}
	}
    
}

