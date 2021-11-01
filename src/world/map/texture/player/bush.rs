use worldlib::positions::Coordinate;
use engine::{
    tetra::{
        Context,
        graphics::{
            Texture,
            Rectangle,
        },
    },
    graphics::{byte_texture, position},
};

use crate::world::RenderCoords;

#[derive(Default)]
pub struct PlayerBushTexture {
	instances: Vec<BushRustle>,
    pub in_bush: bool,
}

struct BushRustle {
	counter: f32,
    coords: Coordinate,
	texture: &'static Texture,
}

impl BushRustle {
    pub fn new(coords: Coordinate) -> Self {
        Self {
            counter: 0.0,
            coords,
            texture: bush_rustle(),
        }
    }
}

static mut BUSH_TEXTURE: Option<Texture> = None;

pub (crate) fn new(ctx: &mut Context) {
    unsafe { BUSH_TEXTURE = Some(byte_texture(ctx, include_bytes!("../../../../../assets/world/textures/player/bush_temp.png"))) }
}

fn bush_rustle() -> &'static Texture {
	unsafe { BUSH_TEXTURE.as_ref().unwrap() }
}

impl PlayerBushTexture {
    pub fn add(&mut self, coords: Coordinate) {
        self.instances.push(BushRustle::new(coords));
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
            rustle.texture.draw_region(
                ctx,
                Rectangle::new(
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
                    16.0
                ),
                position(x, y)
            );
        }

    }
}
