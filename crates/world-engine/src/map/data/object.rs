use worldlib::{
    map::object::{ItemObject, MapObject, ObjectId},
    positions::{Coordinate, Location},
    state::WorldState,
    TILE_SIZE,
};

use crate::{
    engine::{
        error::ImageError,
        graphics::{DrawParams, Texture},
        math::Rectangle,
        Context,
        utils::HashMap,
    },
};

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
    pub fn new(ctx: &mut Context, objects: HashMap<ObjectId, Vec<u8>>) -> Result<Self, ImageError> {
        let mut textures = HashMap::with_capacity(objects.len());
        for (id, data) in objects {
            let texture = Texture::new(ctx, &data)?;
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
        ctx: &mut Context,
        map: &Location,
        objects: &HashMap<Coordinate, MapObject>,
        items: &HashMap<Coordinate, ItemObject>,
        world: &WorldState,
        screen: &RenderCoords,
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
                texture.draw(
                    ctx,
                    x,
                    y,
                    DrawParams {
                        source: Some(Rectangle {
                            x: 0.0,
                            y: 0.0,
                            w: TILE_SIZE,
                            h: TILE_SIZE,
                        }),
                        ..Default::default()
                    },
                )
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
            const BALL: &ObjectId = unsafe { &ObjectId::new_unchecked(1819042146) };

            if let Some(texture) = self.textures.get(BALL) {
                let x = ((coords.x + screen.offset.x) << 4) as f32 - screen.focus.x;
                let y = ((coords.y + screen.offset.y) << 4) as f32 - screen.focus.y;
                texture.draw(ctx, x, y, Default::default())
            }
        }
        for anim in self.active.iter() {
            anim.draw(ctx, screen);
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

    pub fn draw(&self, ctx: &mut Context, screen: &RenderCoords) {
        let x = ((self.coordinate.x + screen.offset.x) << 4) as f32 - screen.focus.x;
        let y = ((self.coordinate.y + screen.offset.y) << 4) as f32 - screen.focus.y;
        self.texture.draw(
            ctx,
            x,
            y,
            DrawParams {
                source: Some(Rectangle {
                    x: 16.0 + self.accumulator.floor() * 16.0,
                    y: 0.0,
                    w: TILE_SIZE,
                    h: TILE_SIZE,
                }),
                ..Default::default()
            },
        )
    }
}
