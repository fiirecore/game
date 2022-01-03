use worldlib::{map::WorldMap, positions::Coordinate};

use crate::engine::{
    error::ImageError,
    graphics::{DrawParams, Texture},
    math::Rectangle,
    Context,
};

use crate::map::RenderCoords;

pub struct PlayerBushTexture {
    pub texture: Texture,
    instances: Vec<BushRustle>,
    pub in_bush: bool,
}

impl PlayerBushTexture {
    pub fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            texture: Texture::new(
                ctx,
                include_bytes!("../../../../assets/textures/bush_temp.png"),
            )?,
            instances: Vec::new(),
            in_bush: false,
        })
    }

    pub fn check(&mut self, map: &WorldMap, coords: Coordinate) {
        self.in_bush = map.tile(coords) == Some(0x0D);
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
    pub fn draw(&self, ctx: &mut Context, screen: &RenderCoords) {
        for rustle in self.instances.iter() {
            let x = ((rustle.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x;
            let y = ((rustle.coords.y + screen.offset.y) << 4) as f32 - screen.focus.y;
            rustle.texture.draw(
                ctx,
                x,
                y,
                DrawParams::source(Rectangle::new(
                    0.0,
                    if rustle.counter < 0.25 {
                        0.0
                    } else if rustle.counter < 0.5 {
                        16.0
                    } else if rustle.counter < 0.75 {
                        32.0
                    } else {
                        48.0
                    },
                    16.0,
                    16.0,
                )),
            );
        }
    }
}
