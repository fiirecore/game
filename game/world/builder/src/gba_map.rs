use worldlib::{MapSize, TileId, MovementId};

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