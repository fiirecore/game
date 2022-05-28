use pokengine::engine::{
    graphics::{Color, Draw, DrawExt, DrawImages, DrawParams, Graphics},
    math::Rect,
};
use worldlib::{
    map::object::{ItemObject, MapObject, ObjectId},
    positions::{Coordinate, Location},
    state::WorldState,
    TILE_SIZE,
};

use crate::engine::{graphics::Texture, utils::HashMap};

use crate::map::RenderCoords;

pub struct ObjectTextures {
    textures: HashMap<ObjectId, Texture>,
    active: Vec<ObjectAnimation>,
}

struct ObjectAnimation {
    texture: Texture,
    coordinate: Coordinate,
    accumulator: f32,
}

impl ObjectTextures {
    pub fn new(gfx: &mut Graphics, objects: HashMap<ObjectId, Vec<u8>>) -> Result<Self, String> {
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

    pub fn add(&mut self, coordinate: Coordinate, id: &ObjectId) {
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

    pub fn draw(
        &self,
        draw: &mut Draw,
        map: &Location,
        objects: &HashMap<Coordinate, MapObject>,
        items: &HashMap<Coordinate, ItemObject>,
        world: &WorldState,
        screen: &RenderCoords,
        color: Color,
    ) {
        for (coords, object) in objects.iter().filter(|(coordinate, ..)| {
            !world
                .objects
                .get(map)
                .map(|coords| coords.contains(coordinate))
                .unwrap_or_default()
        }) {
            if let Some(texture) = self.textures.get(&object.group) {
                let x = ((coords.x + screen.offset.x) << 4) as f32 - screen.focus.x;
                let y = ((coords.y + screen.offset.y) << 4) as f32 - screen.focus.y;
                draw.texture(
                    texture,
                    x,
                    y,
                    DrawParams {
                        source: Some(Rect {
                            x: 0.0,
                            y: 0.0,
                            width: TILE_SIZE,
                            height: TILE_SIZE,
                        }),
                        color,
                        ..Default::default()
                    },
                );
                // texture.draw(
                //     ctx,
                //     x,
                //     y,
                //     DrawParams {
                //         source: Some(Rectangle {
                //             x: 0.0,
                //             y: 0.0,
                //             w: TILE_SIZE,
                //             h: TILE_SIZE,
                //         }),
                //         color,
                //         ..Default::default()
                //     },
                // )
            }
        }
        for (coords, ..) in items.iter().filter(|(coordinate, item)| {
            !item.hidden
                || !world
                    .objects
                    .get(map)
                    .map(|coords| coords.contains(coordinate))
                    .unwrap_or_default()
        }) {
            /// "ball"
            const BALL: &ObjectId =
                unsafe { &ObjectId::from_bytes_unchecked(1819042146u64.to_ne_bytes()) };

            if let Some(texture) = self.textures.get(BALL) {
                let x = ((coords.x + screen.offset.x) << 4) as f32 - screen.focus.x;
                let y = ((coords.y + screen.offset.y) << 4) as f32 - screen.focus.y;
                draw.image(texture).position(x, y).color(color);
            }
        }
        for anim in self.active.iter() {
            anim.draw(draw, screen, color);
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

    pub fn draw(&self, draw: &mut Draw, screen: &RenderCoords, color: Color) {
        let x = ((self.coordinate.x + screen.offset.x) << 4) as f32 - screen.focus.x;
        let y = ((self.coordinate.y + screen.offset.y) << 4) as f32 - screen.focus.y;
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
