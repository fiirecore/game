use pokengine::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, Graphics},
    math::Rect,
};
use worldlib::{map::object::ObjectType, positions::Coordinate, state::map::MapState, TILE_SIZE};

use crate::engine::{graphics::Texture, HashMap};

use crate::map::CharacterCamera;

pub struct ObjectTextures {
    textures: HashMap<ObjectType, Texture>,
    active: Vec<ObjectAnimation>,
}

struct ObjectAnimation {
    texture: Texture,
    coordinate: Coordinate,
    accumulator: f32,
}

impl ObjectTextures {
    pub fn new(gfx: &mut Graphics, objects: HashMap<ObjectType, Vec<u8>>) -> Result<Self, String> {
        let mut textures = HashMap::with_capacity(objects.len());
        for (id, data) in objects {
            let texture = gfx.create_texture().from_image(&data).build()?;
            textures.insert(id, texture);
        }
        Ok(Self {
            textures,
            active: Default::default(),
        })
    }

    pub fn add(&mut self, coordinate: Coordinate, id: &ObjectType) {
        if let Some(texture) = self.textures.get(id).cloned() {
            self.active.push(ObjectAnimation::new(coordinate, texture));
        }
    }

    pub fn update(&mut self, delta: f32) {
        for anim in self.active.iter_mut() {
            anim.update(delta);
        }
        while self
            .active
            .get(0)
            .map(|anim| anim.finished())
            .unwrap_or_default()
        {
            self.active.remove(0);
        }
    }

    pub fn draw(&self, draw: &mut Draw, state: &MapState, camera: &CharacterCamera, color: Color) {
        if let Some(state) = state.entities.get(&state.location) {
            // for object in state.objects.values().filter(|object| !object.removed) {
            //     if let Some(texture) = self.textures.get(&object.entity.data.group) {
            //         let x = ((object.entity.coordinate.x + camera.offset.x) << 4) as f32
            //             - camera.focus.x;
            //         let y = ((object.entity.coordinate.y + camera.offset.y) << 4) as f32
            //             - camera.focus.y;
            //         draw.texture(
            //             texture,
            //             x,
            //             y,
            //             DrawParams {
            //                 source: Some(Rect {
            //                     x: 0.0,
            //                     y: 0.0,
            //                     width: TILE_SIZE,
            //                     height: TILE_SIZE,
            //                 }),
            //                 color,
            //                 ..Default::default()
            //             },
            //         );
            //         // texture.draw(
            //         //     ctx,
            //         //     x,
            //         //     y,
            //         //     DrawParams {
            //         //         source: Some(Rectangle {
            //         //             x: 0.0,
            //         //             y: 0.0,
            //         //             w: TILE_SIZE,
            //         //             h: TILE_SIZE,
            //         //         }),
            //         //         color,
            //         //         ..Default::default()
            //         //     },
            //         // )
            //     }
            // }
            // for object in state
            //     .items
            //     .values()
            //     .filter(|object| !object.removed || !object.entity.data.hidden)
            // {
            //     /// "ball"
            //     const BALL: &ObjectType =
            //         unsafe { &ObjectType::from_bytes_unchecked(1819042146u32.to_ne_bytes()) };

            //     if let Some(texture) = self.textures.get(BALL) {
            //         let x = ((object.entity.coordinate.x + camera.offset.x) << 4) as f32
            //             - camera.focus.x;
            //         let y = ((object.entity.coordinate.y + camera.offset.y) << 4) as f32
            //             - camera.focus.y;
            //         draw.image(texture).position(x, y).color(color);
            //     }
            // }
        }

        for anim in self.active.iter() {
            anim.draw(draw, camera, color);
        }
    }
}

impl ObjectAnimation {
    const UPS: f32 = 4.0;
    const FRAMES: f32 = 3.0;

    pub fn new(coordinate: Coordinate, texture: Texture) -> Self {
        Self {
            texture,
            coordinate,
            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta * Self::UPS;
    }

    pub fn finished(&self) -> bool {
        self.accumulator > Self::FRAMES
    }

    pub fn draw(&self, draw: &mut Draw, camera: &CharacterCamera, color: Color) {
        let x = ((self.coordinate.x + camera.offset.x) << 4) as f32 - camera.focus.x;
        let y = ((self.coordinate.y + camera.offset.y) << 4) as f32 - camera.focus.y;
        draw.texture(
            &self.texture,
            x,
            y,
            DrawParams {
                source: Some(Rect {
                    x: 16.0 + self.accumulator.floor() * 16.0,
                    y: 0.0,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                }),
                color,
                ..Default::default()
            },
        );
    }
}
