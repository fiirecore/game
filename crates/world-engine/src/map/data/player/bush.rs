use worldlib::{
    map::{WorldMap, WorldTile},
    positions::Coordinate,
    TILE_SIZE,
};

use crate::engine::{
    graphics::{Draw, DrawExt, DrawParams, Graphics, Texture},
    math::Rect,
};

use crate::map::CharacterCamera;

pub struct PlayerBushTexture {
    pub texture: Texture,
    instances: Vec<BushRustle>,
    pub in_bush: bool,
}

impl PlayerBushTexture {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            texture: gfx
                .create_texture()
                .from_image(include_bytes!("../../../../assets/textures/bush_temp.png"))
                .build()?,
            instances: Vec::new(),
            in_bush: false,
        })
    }

    pub fn check(&mut self, map: &WorldMap, coords: Coordinate) {
        self.in_bush = map.tile(coords).map(WorldTile::id) == Some(0x0D);
        if self.in_bush {
            self.add(coords);
        }
    }
}

struct BushRustle {
    counter: f32,
    coords: Coordinate,
    texture: Texture,
}

impl BushRustle {
    pub fn new(coords: Coordinate, texture: Texture) -> Self {
        Self {
            counter: 0.0,
            coords,
            texture,
        }
    }
}

impl PlayerBushTexture {
    pub fn add(&mut self, coords: Coordinate) {
        self.instances
            .push(BushRustle::new(coords, self.texture.clone()));
    }
    pub fn update(&mut self, delta: f32) {
        for (index, rustle) in self.instances.iter_mut().enumerate() {
            rustle.counter += delta;
            if rustle.counter > 1.0 {
                self.instances.remove(index);
                return;
            }
        }
    }
    pub fn draw(&self, draw: &mut Draw, camera: &CharacterCamera) {
        for rustle in self.instances.iter() {
            let x = ((rustle.coords.x + camera.offset.x) << 4) as f32 - camera.focus.x;
            let y = ((rustle.coords.y + camera.offset.y) << 4) as f32 - camera.focus.y;
            let yimg = if rustle.counter < 0.25 {
                0.0
            } else if rustle.counter < 0.5 {
                16.0
            } else if rustle.counter < 0.75 {
                32.0
            } else {
                48.0
            };
            draw.texture(
                &rustle.texture,
                x,
                y,
                DrawParams {
                    source: Some(Rect {
                        x: 0.0,
                        y: yimg,
                        width: TILE_SIZE,
                        height: TILE_SIZE,
                    }),
                    ..Default::default()
                },
            );
        }
    }
}
