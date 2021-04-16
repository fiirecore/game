use std::ops::AddAssign;
use std::path::Path;
use std::path::PathBuf;

use ahash::AHashMap as HashMap;
use firecore_world_lib::{MapSize, TileId, MovementId};
use image::GenericImageView;

pub struct GbaMap {
	
	pub music: u8,
	pub width: MapSize,
	pub height: MapSize,
	pub palettes: [u8; 2],
	pub borders: [TileId; 4],
	pub tiles: Vec<TileId>,
	pub movements: Vec<MovementId>,
	
}

pub fn get_gba_map(file: Vec<u8>) -> GbaMap  {

	let bytes = file;

	let music = bytes[40];

	let width = bytes[0] as usize; // 0 - 3 reserved
	let height = bytes[4] as usize; // 4 - 7 reserved
	
	let palettes = [bytes[8], bytes[12]]; // 8 - 11 reserved & 12 - 15 reserved
	
	//let show_name_on_entering = bytes[49];
	
	let mut borders: [u16; 4] = [0; 4];
	
	for x in 0..4 {
		
		let location = 52+x*2;
		
		let tile_num = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
		
		borders[x] = tile_num;
		
	}

	let size = width * height;
	
	let mut tiles: Vec<TileId> = Vec::with_capacity(size);
	let mut movements: Vec<MovementId> = Vec::with_capacity(size);
	
	for tile in 0..size {

		let location = 60 + tile * 2;

		let tile = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
	
		let movement = (bytes[location+1]/4) as u8;
		
		tiles.push(tile);
		movements.push(movement);
		
	}

	GbaMap {
		
		music: music,
		width: width as MapSize,
		height: height as MapSize,
		palettes: palettes,
		borders: borders,
		tiles,
		movements,
		
	}
	
}

pub fn fix_tiles(gba_map: &mut GbaMap, palette_sizes: &HashMap<u8, u16>) {

	let offset = get_offset(gba_map, palette_sizes);

	let zero_size = *palette_sizes.get(&0).unwrap();
	
	for tile in gba_map.tiles.iter_mut() {
		if *tile > zero_size {
			tile.add_assign(offset);
		}
	}

	for index in 0..gba_map.borders.len() {
		if gba_map.borders[index] > zero_size {
			gba_map.borders[index] += offset;
		}
	}

	if gba_map.palettes[0] > 0 {
		let mut offset12: u16 = 0;

		for x in 0..gba_map.palettes[0] {
			offset12 += *palette_sizes.get(&x).unwrap();
		}

		for tile in gba_map.tiles.iter_mut() {
			if *tile < zero_size {
				tile.add_assign(offset12);
			}
		}

		for index in 0..gba_map.borders.len() {
			if gba_map.borders[index] < zero_size {
				gba_map.borders[index] += offset12;
			}
		}
	}

}

pub fn get_offset(gba_map: &GbaMap, palette_sizes: &HashMap<u8, u16>) -> u16 { // To - do: change to recursive function
	let mut offset = 0;
	if gba_map.palettes[1] >= palette_sizes.len() as u8 {
		eprintln!("Not enough palettes to support gba map textures. Need palette #{}", gba_map.palettes[1]);
		return 0;
	}
	for x in 1..gba_map.palettes[1] {
		offset += palette_sizes.get(&x).unwrap();
	}
	return offset;
}

pub fn fill_palette_map<P: AsRef<Path>>(tile_textures: P) -> (HashMap<u8, u16>, HashMap<u8, Vec<u8>>) {
	let mut sizes = HashMap::new();
	let mut palettes = HashMap::new();

	if let Ok(dir) = std::fs::read_dir(tile_textures) {
		let paths: Vec<PathBuf> = dir.filter(|entry| entry.is_ok()).map(|entry| entry.unwrap().path()).filter(|path| path.is_file()).collect();
		for filepath in paths {
			let filename = filepath.file_name().unwrap().to_string_lossy();
			if filename.starts_with("P") {
				if filename.ends_with("B.png") {
					match filename[7..filename.len()-5].parse::<u8>() {
						Ok(index) => {
							match std::fs::read(&filepath) {
							    Ok(bytes) => {
									let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).unwrap();
									sizes.insert(index, ((img.width() >> 4) * (img.height() >> 4)) as u16);
									palettes.insert(index, bytes);
								}
							    Err(err) => {
									panic!("Could not read image at path {:?} with error {}", filepath, err);
								}
							}
						}
						Err(err) => {
							panic!("Could not parse tile palette named {} with error {}", filename, err);
						}
					}
				}
			}
		}
	}

	(sizes, palettes)

}