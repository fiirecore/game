use world::map::{MovementId, TileId};

#[derive(Debug, Clone)]
pub struct BinaryMap {
    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,
    pub border: BinaryMapBorder,
}

#[derive(Debug, Clone)]
pub struct BinaryMapBorder {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileId>,
}

impl BinaryMap {
    pub fn load(map: &[u8], borders: &[u8], size: usize) -> Option<Self> {
        let mut border: [u16; 4] = [0; 4];

        for i in 0..border.len() {
            let location = i * 2;

            let tile_num =
                (*borders.get(location + 1)? % 4) as u16 * 256 + *borders.get(location)? as u16;

            border[i] = tile_num;
        }

        let mut tiles: Vec<TileId> = Vec::with_capacity(size);
        let mut movements: Vec<MovementId> = Vec::with_capacity(size);

        for tile in 0..size {
            let location = tile * 2;

            let tile = (map[location + 1] % 4) as u16 * 256 + map[location] as u16;

            let movement = (map[location + 1] / 4) as u8;

            tiles.push(tile);
            movements.push(movement);
        }

        Some(Self {
            tiles,
            movements,
            border: BinaryMapBorder {
                width: 2,
                height: 2,
                tiles: border.into(),
            },
        })
    }

    pub fn map_music(music: u8) -> Result<tinystr::TinyStr16, GbaMapError> {
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
            0x39 => "vermilion",
            0x1F => "viridian_forest",
            id => return Err(GbaMapError::MusicMapping(id)),
        };
        let m = m.parse().unwrap_or_else(|err| {
            panic!("Could not get map music id from {} with error {}", m, err)
        });
        Ok(m)
    }
}

#[derive(Debug)]
pub enum GbaMapError {
    MusicMapping(u8),
    EndOfFile,
}

impl std::error::Error for GbaMapError {}

impl std::fmt::Display for GbaMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GbaMapError::MusicMapping(music) => write!(
                f,
                "Could not get map music id from {} because it has not been added yet!",
                music,
            ),
            GbaMapError::EndOfFile => write!(f, "Reached end of file too soon!"),
        }
    }
}
