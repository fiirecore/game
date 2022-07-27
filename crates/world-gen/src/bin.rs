use firecore_world::map::{MovementId, MusicId, TileId};

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

}