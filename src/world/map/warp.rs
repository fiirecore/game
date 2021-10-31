use hashbrown::HashMap;
use worldlib::{
    map::{manager::WorldMapManager, TileId, World},
    positions::Coordinate,
};

use crate::{
    engine::{
        graphics::draw_rectangle,
        tetra::{
            graphics::{Color, Texture},
            Context,
        },
        util::{Entity, Reset, HEIGHT, WIDTH},
        EngineContext,
    },
    world::RenderCoords,
};

pub struct WarpTransition {
    alive: bool,
    door: Option<Door>,
    pub doors: HashMap<TileId, Texture>,
    color: Color,
    // rect_width: f32,
    faded: bool,
    warped: bool,
    warp: Option<(Coordinate, bool)>, //coords, move_on_exit
    freeze: bool,
}

pub struct Door {
    pub tile: TileId,
    pub coords: Coordinate,
    pub open: bool,
    pub accumulator: f32,
}

impl Door {
    pub const DOOR_MAX: f32 = 3.99;
    pub fn new(tile: TileId, coords: Coordinate) -> Self {
        Self {
            tile,
            coords,
            open: false,
            accumulator: 0.0,
        }
    }
}

impl WarpTransition {
    // const RECT_WIDTH: f32 = WIDTH / 2.0;

    pub fn new() -> Self {
        Self {
            alive: false,
            door: None,
            doors: HashMap::new(),
            color: Color::BLACK,
            // rect_width: Self::RECT_WIDTH,
            faded: false,
            warped: false,
            warp: None,
            freeze: false,
        }
    }

    pub fn update(&mut self, world: &mut WorldMapManager, delta: f32) -> Option<bool> {
        // returns map change

        match self.faded {
            true => {
                self.color.a -= delta * 3.0;
                if self.color.a < 0.0 {
                    self.color.a = 0.0;
                    self.faded = false;
                    if self.warped {
                        let coords = self.warp.as_ref().unwrap().0;
                        let tile = world.tile(coords).unwrap_or_default();
                        if self.doors.contains_key(&tile) {
                            //exit door
                            self.door = Some(Door::new(tile, coords));
                        } else if self.warp.as_ref().unwrap().1 {
                            world.player.hidden = false;
                            let direction = world.player.position.direction;
                            world.player.pathing.queue.push(direction);
                        }
                    }
                }
            }
            false => match &mut self.door {
                Some(door) => match door.open {
                    true => {
                        if !world.player.moving() && door.accumulator >= 0.0 {
                            if door.accumulator == Door::DOOR_MAX && !self.warped {
                                world.player.hidden = true;
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
                                    // world.try_move(world.player.position.direction, delta);
                                    let direction = world.player.position.direction;
                                    world.player.pathing.queue.push(direction);
                                }
                                door.open = true;
                                if self.warped {
                                    world.player.hidden = false;
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
                            if let Some(destination) = world.warp.take() {
                                world.player.hidden = destination.transition.move_on_exit;
                                let change_music = destination.transition.change_music;
                                world.warp(destination);
                                self.warp = Some((
                                    destination.position.coords,
                                    destination.transition.move_on_exit,
                                ));
                                self.warped = true;
                                return Some(change_music);
                            }
                        }
                    }
                    true => {
                        self.despawn();
                        world.player.unfreeze();
                        world.player.input_frozen = self.freeze;
                        // if let Some(destination) = self.warp.take() {
                        //     if destination.transition.move_on_exit {
                        //         world.try_move(
                        //             destination
                        //                 .position
                        //                 .direction
                        //                 .unwrap_or(world.player.position.direction),
                        //             delta,
                        //         );
                        //     }
                        // }
                    }
                },
            },
        }
        None
    }

    pub fn draw(&self, ctx: &mut EngineContext) {
        if self.alive {
            draw_rectangle(ctx, 0.0, 0.0, WIDTH, HEIGHT, self.color);
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

    pub fn draw_door(&self, ctx: &mut Context, screen: &RenderCoords) {
        if self.alive {
            if let Some(door) = &self.door {
                use engine::graphics::position;
                use engine::tetra::graphics::Rectangle;
                use worldlib::TILE_SIZE;
                if let Some(texture) = self.doors.get(&door.tile) {
                    texture.draw_region(
                        ctx,
                        Rectangle::new(
                            0.0,
                            door.accumulator.floor() * TILE_SIZE,
                            TILE_SIZE,
                            TILE_SIZE,
                        ),
                        position(
                            ((door.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x,
                            ((door.coords.y + screen.offset.y) << 4) as f32 - screen.focus.y,
                        ),
                    )
                }
            }
        }
    }

    pub fn queue(&mut self, world: &mut WorldMapManager, tile: TileId, coords: Coordinate) {
        if self.doors.contains_key(&tile) {
            // enterance door
            self.door = Some(Door::new(tile, coords));
            self.freeze = world.player.input_frozen;
            world.player.input_frozen = true;
            self.spawn();
        }
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

impl Reset for WarpTransition {
    fn reset(&mut self) {
        self.color.a = 0.0;
        // self.rect_width = Self::RECT_WIDTH;
        // self.switch = false;
        self.faded = false;
        self.warped = false;
    }
}
