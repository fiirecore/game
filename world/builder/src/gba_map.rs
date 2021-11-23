use worldlib::map::{MapSize, MovementId, TileId};

pub struct GbaMap {
    pub music: u8,
    pub width: MapSize,
    pub height: MapSize,
    pub palettes: [u8; 2],
    pub borders: [TileId; 4],
    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,
}

pub fn get_gba_map(file: Vec<u8>) -> GbaMap {
    let bytes = file;

    let music = bytes[40];

    let width = bytes[0] as usize; // 0 - 3 reserved
    let height = bytes[4] as usize; // 4 - 7 reserved

    let palettes = [bytes[8], bytes[12]]; // 8 - 11 reserved & 12 - 15 reserved

    //let show_name_on_entering = bytes[49];

    let mut borders: [u16; 4] = [0; 4];

    for (i, t) in borders.iter_mut().enumerate() {
        let location = 52 + i * 2;

        let tile_num = (bytes[location + 1] % 4) as u16 * 256 + bytes[location] as u16;

        *t = tile_num;
    }

    let size = width * height;

    let mut tiles: Vec<TileId> = Vec::with_capacity(size);
    let mut movements: Vec<MovementId> = Vec::with_capacity(size);

    for tile in 0..size {
        let location = 60 + tile * 2;

        let tile = (bytes[location + 1] % 4) as u16 * 256 + bytes[location] as u16;

        let movement = (bytes[location + 1] / 4) as u8;

        tiles.push(tile);
        movements.push(movement);
    }

    GbaMap {
        music,
        width: width as MapSize,
        height: height as MapSize,
        palettes,
        borders,
        tiles,
        movements,
    }
}

pub fn get_gba_music(music: u8) -> tinystr::TinyStr16 {
    let m = match music {
        0x35 => "celadon",
        0x17 => "cinnabar",
        0x34 => "fuchsia",
		0x13 => "gym",
		0x18 => "lavender",
		0x20 => "mt_moon",
		0x2D => "oaks_lab",
		0x2C => "pallet",
		0x3A => "pewter",
		0x2F => "pokemon_center",
		0x23 => "route_1",
		0x24 => "route_2",
		0x25 => "route_3",
		0x26 => "route_4",
		0x39 => "vermillion",
		0x1F => "viridian_forest",
		_ => panic!("Could not get map music id from {} because it has not been added yet!", music),
    };
	let m = m.parse().unwrap_or_else(|err| panic!("Could not get map music id from {} with error {}", m, err));
	m
}
