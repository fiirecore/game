use worldlib::{
    character::{action::ActionQueue, player::PlayerCharacter},
    map::{
        data::{tile::WarpTile, WorldMapData},
        PaletteId, TileId,
    },
    positions::Coordinate,
    state::map::MapState,
};

use crate::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, DrawShapes},
    math::Rect,
    utils::Entity,
};

use crate::map::CharacterCamera;

use super::data::tile::PaletteTextureManager;

pub struct WarpTransition {
    alive: bool,

    door: Option<Door>,
    color: Color,
    // rect_width: f32,
    faded: bool,
    warped: bool,
    warp: Option<(Coordinate, bool)>, //coords, move_on_exit
    freeze: bool,
}

pub struct Door {
    pub palette: PaletteId,
    pub tile: TileId,
    pub coords: Coordinate,
    pub open: bool,
    pub accumulator: f32,
}

impl Door {
    pub const DOOR_MAX: f32 = 3.99;
    pub fn new(palette: PaletteId, tile: TileId, coords: Coordinate) -> Self {
        Self {
            palette,
            tile,
            coords,
            open: false,
            accumulator: 0.0,
        }
    }
}

impl WarpTransition {
    // const RECT_WIDTH: f32 = WIDTH / 2.0;

    pub fn update(
        &mut self,
        world: &WorldMapData,
        state: &mut MapState,
        delta: f32,
    ) -> Option<bool> {
        // returns map change
        let character = &mut state.player.character;

        match self.faded {
            false => match &mut self.door {
                Some(door) => match door.open {
                    true => {
                        if !character.moving() && door.accumulator >= 0.0 {
                            if door.accumulator == Door::DOOR_MAX && !self.warped {
                                character.hidden = true;
                            }
                            door.accumulator -= delta * 6.0;
                            if door.accumulator <= 0.0 {
                                door.accumulator = 0.0;
                                self.door = None;
                            }
                        }
                    }
                    false => {
                        if door.accumulator < Door::DOOR_MAX {
                            door.accumulator += delta * 6.0;
                            if door.accumulator >= Door::DOOR_MAX {
                                door.accumulator = Door::DOOR_MAX;
                                //door fully open
                                if !self.warped
                                    || self.warp.as_ref().map(|d| d.1).unwrap_or_default()
                                {
                                    // world.try_move(player.position.direction, delta);
                                    let direction = character.position.direction;
                                    character.actions.queue.push(ActionQueue::Move(direction));
                                }
                                door.open = true;
                                if self.warped {
                                    character.hidden = false;
                                }
                            }
                        }
                    }
                },
                None => match self.warped {
                    false => {
                        self.color.a += delta * 2.5;
                        if self.color.a >= 1.0 {
                            self.color.a = 1.0;
                            self.faded = true;
                            if let Some(warp) = state.warp.take() {
                                character.hidden = false; //destination.transition.move_on_exit;
                                let change_music = true; // destination.transition.change_music;
                                world.warp(state, warp);
                                self.warp = Some((
                                    warp.position.coords,
                                    false, //destination.transition.move_on_exit,
                                ));
                                self.warped = true;
                                return Some(change_music);
                            }
                        }
                    }
                    true => {
                        self.despawn();
                        match self.freeze {
                            true => character.input_lock.increment(),
                            false => character.input_lock.decrement(),
                        };
                        // if let Some(destination) = self.warp.take() {
                        //     if destination.transition.move_on_exit {
                        //         world.try_move(
                        //             destination
                        //                 .position
                        //                 .direction
                        //                 .unwrap_or(player.position.direction),
                        //             delta,
                        //         );
                        //     }
                        // }
                    }
                },
            },
            true => {
                self.color.a -= delta * 3.0;
                if self.color.a < 0.0 {
                    self.color.a = 0.0;
                    self.faded = false;
                    if self.warped {
                        let coords = self.warp.as_ref().unwrap().0;
                        if let Some((palettes, tile)) = world
                            .maps
                            .get(&state.location)
                            .map(|map| map.tile(coords).map(|tile| (&map.palettes, tile)))
                            .flatten()
                        {
                            let palette = tile.palette(palettes);
                            if let Some((palette, tile, warptile)) = world
                                .palettes
                                .get(palette)
                                .map(|data| {
                                    data.warp
                                        .get(&tile.id())
                                        .map(|warptile| (palette, tile.id(), warptile))
                                })
                                .flatten()
                            {
                                match warptile {
                                    WarpTile::Door => {
                                        character.hidden = true;
                                        //exit door
                                        self.door = Some(Door::new(*palette, tile, coords));
                                    }
                                    WarpTile::Stair | WarpTile::Other => {
                                        character.hidden = false;
                                        let direction = character.position.direction;
                                        character.actions.queue.push(ActionQueue::Move(direction));
                                    }
                                };
                            } else {
                                character.hidden = false;
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn draw(&self, draw: &mut Draw) {
        if self.alive {
            draw.rect((0.0, 0.0), (draw.width(), draw.height()))
                .color(Color::new(
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    self.color.a,
                ));
            // if self.switch {
            // draw_rectangle(ctx, 0.0, 0.0, self.rect_width, HEIGHT, Color::BLACK);
            // draw_rectangle(
            //     ctx,
            //     WIDTH - self.rect_width,
            //     0.0,
            //     self.rect_width,
            //     HEIGHT,
            //     Color::BLACK,
            // );
            // }
        }
    }

    pub fn draw_door(
        &self,
        draw: &mut Draw,
        palettes: &PaletteTextureManager,
        camera: &CharacterCamera,
    ) {
        if self.alive {
            if let Some(door) = &self.door {
                use worldlib::TILE_SIZE;
                if let Some(texture) = palettes
                    .palettes
                    .get(&door.palette)
                    .map(|p| p.doors.get(&door.tile))
                    .flatten()
                {
                    draw.texture(
                        texture,
                        ((door.coords.x + camera.offset.x) << 4) as f32 - camera.focus.x,
                        ((door.coords.y + camera.offset.y) << 4) as f32 - camera.focus.y,
                        DrawParams {
                            source: Some(Rect {
                                x: 0.0,
                                y: door.accumulator.floor() * TILE_SIZE,
                                width: TILE_SIZE,
                                height: TILE_SIZE,
                            }),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    pub fn queue(
        &mut self,
        world: &WorldMapData,
        player: &mut PlayerCharacter,
        palette: PaletteId,
        tile: TileId,
        coords: Coordinate,
    ) {
        if let Some(data) = world.palettes.get(&palette) {
            if let Some(warptile) = data.warp.get(&tile) {
                // entrance door
                match warptile {
                    WarpTile::Door => {
                        self.door = Some(Door::new(palette, tile, coords));
                        self.freeze = player.character.input_lock.active();
                        player.character.input_lock.increment();
                        self.spawn();
                    }
                    _ => (),
                    // WarpTile::Stair => todo!(),
                    // WarpTile::Other => todo!(),
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.color.a = 0.0;
        // self.rect_width = Self::RECT_WIDTH;
        // self.switch = false;
        self.faded = false;
        self.warped = false;
    }
}

impl Entity for WarpTransition {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Default for WarpTransition {
    fn default() -> Self {
        Self {
            alive: false,
            door: None,
            color: Color::BLACK,
            faded: false,
            warped: false,
            warp: None,
            freeze: false,
        }
    }
}
