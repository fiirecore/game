use std::sync::Arc;

use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, Draw, DrawTextSection},
    gui::MessageBox,
    math::{ivec2, IVec2},
    music, sound,
    utils::Entity,
    App, Plugins,
};

use pokengine::pokedex::{Dex, pokemon::Pokemon, moves::Move, item::Item};
use rand::Rng;

use worldlib::{
    character::player::PlayerCharacter,
    map::{
        chunk::Connection,
        manager::{InputEvent, WorldMapManager},
        movement::Elevation,
        warp::WarpDestination,
        Brightness, WorldMap,
    },
    pokedex::trainer::{InitTrainer, Trainer},
    positions::{Coordinate, Destination, Direction, Location, Spot},
    random::WorldRandoms,
    script::WorldScriptingEngine,
    state::{
        map::{MapEvent, MapState},
        WorldState,
    },
};

use crate::map::{data::ClientWorldData, input::PlayerInput, warp::WarpTransition};

pub mod npc;

// pub mod script;

pub struct WorldManager<S: WorldScriptingEngine> {
    pub world: WorldMapManager<S>,

    pub data: ClientWorldData,

    pub warper: WarpTransition,
    pub input: PlayerInput,
}

impl<S: WorldScriptingEngine> WorldManager<S> {

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.world.get(location)
    }

    pub fn start<R: Rng, P, B>(
        &mut self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
    ) {
        if state.location == Location::DEFAULT {
            let Spot { location, position } = self.world.data.spawn;
            self.world.warp(
                state,
                randoms,
                trainer,
                WarpDestination {
                    location,
                    position: position.into(),
                },
            );
        }
        self.world.on_warp(state, randoms, trainer);
    }

    pub fn post_battle(&mut self, state: &mut MapState, trainer: &mut InitTrainer, winner: bool) {
        self.world.data.post_battle(state, trainer, winner)
    }

    pub fn spawn(&self) -> Spot {
        self.world.data.spawn
    }

    pub fn try_teleport<R: Rng, P, B>(
        &mut self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
        location: Location,
    ) -> bool {
        if self.world.contains(&location) {
            self.teleport(state, randoms, trainer, location);
            true
        } else {
            false
        }
    }

    pub fn update<R: Rng>(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        state: &mut WorldState<S>,
        randoms: &mut WorldRandoms<R>,
        trainer: &mut InitTrainer,
        delta: f32,
    ) {
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }

        self.data.update(delta, &mut state.map.player.character);

        if self.warper.alive() {
            if let Some(music) = self.warper.update(&self.world.data, &mut state.map, delta) {
                self.world.on_warp(&mut state.map, randoms, trainer);
            }
        } else if state.map.warp.is_some() {
            self.warper.spawn();
            state.map.player.character.input_lock.increment();
        }

        if let Some(direction) = self
            .input
            .update(app, plugins, &mut state.map.player, delta)
        {
            self.world
                .input(&mut state.map, InputEvent::Move(direction));
        }

        if pressed(app, plugins, Control::A) && !state.map.player.character.input_lock.active() {
            self.world.input(&mut state.map, InputEvent::Interact);
        }

        self.world.update(state, trainer, randoms, delta);

        for action in std::mem::take(&mut state.map.events) {
            match action {
                MapEvent::PlayerJump => self.data.player.jump(&mut state.map.player),
                MapEvent::PlayMusic(music) => match music {
                    Some(music) => match music::get_current_music(plugins) {
                        Some(playing) => {
                            if playing != music {
                                music::play_music(app, plugins, &music);
                            }
                        }
                        None => music::play_music(app, plugins, &music),
                    },
                    None => music::stop_music(app, plugins),
                },
                MapEvent::PlaySound(sound, variant) => {
                    sound::play_sound(app, plugins, sound, variant);
                }
                MapEvent::BeginWarpTransition(coords) => {
                    if let Some(map) = self.world.get(&state.map.location) {
                        if let Some(tile) = map.tile(coords) {
                            let palette = *tile.palette(&map.palettes);
                            let tile = tile.id();
                            self.warper.queue(
                                &self.world.data,
                                &mut state.map.player,
                                palette,
                                tile,
                                coords,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn teleport<R: Rng, P, B>(
        &mut self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
        location: Location,
    ) {
        if let Some(map) = self.world.data.maps.get(&location) {
            let coords = map.settings.fly_position.unwrap_or_else(|| {
                let mut count = 0u8;
                let mut first = None;
                let index = match map.movements.iter().enumerate().find(|(i, tile)| {
                    if Elevation::can_move(Elevation(0), **tile) {
                        count += 1;
                        if first.is_none() {
                            first = Some((*i, **tile));
                        }
                        if count == 8 {
                            return true;
                        }
                    }
                    false
                }) {
                    Some((index, ..)) => index,
                    None => first.map(|(index, ..)| index).unwrap_or_default(),
                } as i32;
                let x = index % map.width;
                let y = index / map.width;
                Coordinate { x, y }
            });
            let location = map.id;
            self.world.warp(
                state,
                randoms,
                trainer,
                WarpDestination {
                    location,
                    position: Destination {
                        coords,
                        direction: Some(Direction::Down),
                    },
                },
            );
        }
    }

    pub fn ui(
        &mut self,
        app: &App,
        plugins: &mut Plugins,
        egui: &crate::engine::notan::egui::Context,
        state: &mut MapState,
    ) {
        MessageBox::ui(app, plugins, egui, &mut state.message);
    }

    pub fn draw(&self, draw: &mut Draw, state: &MapState) {
        let camera = super::CharacterCamera::new((draw.width(), draw.height()), &state.player.character);

        let color = match self.world.get(&state.location) {
            Some(current) => {
                let color = match current.settings.brightness {
                    Brightness::Day => Color::WHITE,
                    Brightness::Night => Color::new(0.6, 0.6, 0.6, 1.0),
                };

                super::draw(draw, current, state, &self.data, &camera, true, color);

                match &current.chunk {
                    Some(chunk) => {
                        for (connection, direction, offset) in chunk
                            .connections
                            .iter()
                            .flat_map(|(d, connections)| connections.iter().map(move |c| (d, c)))
                            .flat_map(|(direction, Connection(location, offset))| {
                                self.world
                                    .data
                                    .maps
                                    .get(&location)
                                    .map(|map| (map, direction, offset))
                            })
                        {
                            fn map_offset(
                                direction: &Direction,
                                current: &WorldMap,
                                map: &WorldMap,
                                offset: i32,
                            ) -> IVec2 {
                                match direction {
                                    Direction::Down => ivec2(offset, current.height),
                                    Direction::Up => ivec2(offset, -map.height),
                                    Direction::Left => ivec2(-map.width, offset),
                                    Direction::Right => ivec2(current.width, offset),
                                }
                            }

                            super::draw(
                                draw,
                                connection,
                                state,
                                &self.data,
                                &camera.offset(map_offset(direction, current, connection, *offset)),
                                false,
                                color,
                            );
                        }
                    }
                    None => (),
                }

                color
            }
            None => {
                draw.text(&self.data.debug_font, "Cannot get map:")
                    .position(0.0, 0.0)
                    .color(Color::WHITE)
                    .h_align_left()
                    .v_align_top();
                draw.text(
                    &self.data.debug_font,
                    state.location.map.as_deref().unwrap_or("None"),
                )
                .position(0.0, 8.0)
                .color(Color::WHITE)
                .h_align_left()
                .v_align_top();
                draw.text(&self.data.debug_font, state.location.index.as_str())
                    .position(0.0, 16.0)
                    .color(Color::WHITE)
                    .h_align_left()
                    .v_align_top();
                Color::WHITE
            }
        };

        if state.debug_mode {
            draw.text(
                &self.data.debug_font,
                state
                    .location
                    .map
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("No Base Map ID"),
            )
            .position(5.0, 5.0)
            .color(Color::WHITE)
            .h_align_left()
            .v_align_top();

            draw.text(&self.data.debug_font, state.location.index.as_str())
                .position(5.0, 15.0)
                .color(Color::WHITE)
                .h_align_left()
                .v_align_top();

            let mut coordarr = [0u8; 16];

            use std::io::Write;

            if let Ok(..) = write!(
                &mut coordarr as &mut [u8],
                "{}",
                state.player.character.position.coords
            ) {
                if let Ok(text) = std::str::from_utf8(&coordarr) {
                    draw.text(&self.data.debug_font, text)
                        .position(5.0, 25.0)
                        .color(Color::WHITE);
                }
            }
        } else {
            self.warper.draw_door(draw, &self.data.tiles, &camera);
        }

        self.data.player.draw(draw, &state.player.character, color);
        if !state.debug_mode {
            self.data.player.bush.draw(draw, &camera);
            self.warper.draw(draw);
        }
    }
}

fn on_tile(
    map: &WorldMap,
    player: &PlayerCharacter,
    data: &mut ClientWorldData,
    // sender: &Sender<MapEvent>,
) {
    data.player
        .bush
        .check(map, player.character.position.coords);
}

// fn get_mut(world: &mut WorldMapManager) -> Option<&mut WorldMap> {
//     match world.data.location.as_ref() {
//         Some(cur) => world.maps.get_mut(cur),
//         None => None,
//     }
// }
