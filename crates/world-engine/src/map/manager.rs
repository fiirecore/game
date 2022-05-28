use std::ops::Deref;

use crate::engine::{
    controls::{pressed, Control},
    graphics::{Color, Draw, DrawTextSection, Graphics, Font},
    gui::MessageBox,
    math::{ivec2, IVec2},
    music, sound,
    utils::Entity,
    App, Plugins,
};

use pokengine::pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex};

use rand::prelude::SmallRng;

use worldlib::{
    character::player::{InitPlayerCharacter, PlayerCharacter},
    map::{
        chunk::Connection,
        manager::{InputEvent, WorldMapManager},
        movement::Elevation,
        warp::WarpDestination,
        Brightness, WorldMap,
    },
    positions::{Coordinate, Destination, Direction, Location, Position},
    serialized::SerializedWorld,
    state::WorldEvent,
};

use crate::map::{data::ClientWorldData, input::PlayerInput, warp::WarpTransition, RenderCoords};

pub mod npc;

// pub mod script;

pub struct WorldManager {
    pub world: WorldMapManager<SmallRng>,

    data: ClientWorldData,

    warper: WarpTransition,
    input: PlayerInput,
    // screen: RenderCoords,
    // events: EventReceiver<WorldEvents>,
}

impl WorldManager {
    pub fn new(
        gfx: &mut Graphics,
        world: SerializedWorld,
        debug_font: Font,
    ) -> Result<Self, String> {
        // let events = Default::default();

        Ok(Self {
            world: WorldMapManager::new(world.data, world.scripts),

            data: ClientWorldData::new(gfx, world.textures, debug_font)?,
            warper: WarpTransition::new(),
            // text: TextWindow::new(gfx)?,
            input: PlayerInput::default(),
            // screen: RenderCoords::default(),
            // events,
        })
    }

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.world.get(location)
    }

    pub fn start<P, B: Default>(&mut self, player: &mut PlayerCharacter<P, B>) {
        if player.location == Location::DEFAULT {
            let (location, position) = self.world.data.spawn;
            self.world.warp(
                player,
                WarpDestination {
                    location,
                    position: Destination {
                        coords: position.coords,
                        direction: Some(position.direction),
                    },
                },
            );
        }
        self.world.on_warp(player);
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
    }

    pub fn post_battle<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        winner: bool,
    ) {
        self.world.data.post_battle(player, winner)
    }

    pub fn spawn(&self) -> (Location, Position) {
        self.world.data.spawn
    }

    pub fn try_teleport<P, B: Default>(
        &mut self,
        player: &mut PlayerCharacter<P, B>,
        location: Location,
    ) -> bool {
        if self.world.contains(&location) {
            self.teleport(player, location);
            true
        } else {
            false
        }
    }

    pub fn update<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        player: &mut InitPlayerCharacter<P, M, I>,
        itemdex: &impl Dex<Item, Output = I>,
        delta: f32,
    ) {
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }

        self.data.update(delta, player);

        if self.warper.alive() {
            if let Some(music) = self.warper.update(&self.world.data, player, delta) {
                self.world.on_warp(player);
            }
        } else if player.state.warp.is_some() {
            self.warper.spawn();
            player
                .character
                .input_lock.increment();
        }

        if let Some(direction) = self.input.update(app, plugins, player, delta) {
            self.world.input(player, InputEvent::Move(direction));
        }

        if pressed(app, plugins, Control::A)
            && !player
                .character
                .input_lock.active()
        {
            self.world.input(player, InputEvent::Interact);
        }

        self.world.update(player, itemdex, delta);

        for action in std::mem::take(&mut player.state.events) {
            match action {
                WorldEvent::PlayerJump => self.data.player.jump(player),
                // WorldEvent::GivePokemon(pokemon) => {
                //     if let Some(pokemon) = pokemon.init(
                //         &mut self.data.randoms.general,
                //         crate::pokedex(),
                //         crate::movedex(),
                //         crate::itemdex(),
                //     ) {
                //         party.try_push(pokemon);
                //     }
                // }
                WorldEvent::PlayMusic(music) => {
                    if let Some(playing) = music::get_current_music(plugins) {
                        if playing != music {
                            music::play_music(app, plugins, &music);
                        }
                    } else {
                        music::play_music(app, plugins, &music);
                    }
                }
                WorldEvent::PlaySound(sound, variant) => {
                    sound::play_sound(app, plugins, sound, variant);
                }
                WorldEvent::BeginWarpTransition(coords) => {
                    if let Some(map) = self.world.get(&player.location) {
                        if let Some(tile) = map.tile(coords) {
                            let palette = *tile.palette(&map.palettes);
                            let tile = tile.id();
                            self.warper
                                .queue(&self.world.data, player, palette, tile, coords);
                        }
                    }
                }
                WorldEvent::OnTile => {
                    if let Some(map) = self.world.get(&player.location) {
                        on_tile(map, player, &mut self.data)
                    }
                }
                WorldEvent::BreakObject(coordinate) => {
                    if let Some(map) = self.world.get(&player.location) {
                        if let Some(object) = map.object_at(&coordinate) {
                            self.data.object.add(coordinate, &object.group);
                        }
                    }
                }
            }
        }
    }

    // #[deprecated]
    // fn debug_input(&mut self, ctx: &Context, save: &mut PlayerData) {
    //     if is_key_pressed(ctx, Key::F3) {
    //         info!("Local Coordinates: {}", self.player.position.coords);

    //         match self.world.tile(self.player.position.coords) {
    //             Some(tile) => info!("Current Tile ID: {:x}", tile),
    //             None => info!("Currently out of bounds"),
    //         }

    //         info!("Player is {:?}", self.player.movement);
    //     }

    //     if is_key_pressed(ctx, Key::F5) {
    //         if let Some(map) = self.world.get() {
    //             info!("Resetting battled trainers in this map! ({})", map.name);
    //             save.world.get_map(&map.id).battled.clear();
    //         }
    //     }
    // }

    pub fn teleport<P, B: Default>(
        &mut self,
        player: &mut PlayerCharacter<P, B>,
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
                player,
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

    pub fn ui<P, B: Default>(
        &mut self,
        app: &App,
        plugins: &mut Plugins,
        egui: &crate::engine::notan::egui::Context,
        player: &mut PlayerCharacter<P, B>,
    ) {
        MessageBox::ui(app, plugins, egui, &mut player.state.message);
    }

    pub fn draw<P, B: Default>(&self, draw: &mut Draw, player: &PlayerCharacter<P, B>) {
        let screen = RenderCoords::new(&draw, &player.character);

        let color = match self.world.get(&player.location) {
            Some(current) => {
                let color = match current.settings.brightness {
                    Brightness::Day => Color::WHITE,
                    Brightness::Night => Color::new(0.6, 0.6, 0.6, 1.0),
                };

                super::draw(
                    draw,
                    current,
                    &player.state,
                    &self.data,
                    &screen,
                    true,
                    color,
                );

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
                                &player.state,
                                &self.data,
                                &screen.offset(map_offset(direction, current, connection, *offset)),
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
                    player.location.map.as_deref().unwrap_or("None"),
                )
                .position(0.0, 8.0)
                .color(Color::WHITE)
                .h_align_left()
                .v_align_top();
                draw.text(&self.data.debug_font, player.location.index.as_str())
                    .position(0.0, 16.0)
                    .color(Color::WHITE)
                    .h_align_left()
                    .v_align_top();
                Color::WHITE
            }
        };

        if player.state.debug_draw {
            draw.text(
                &self.data.debug_font,
                player
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

            draw.text(&self.data.debug_font, player.location.index.as_str())
                .position(5.0, 15.0)
                .color(Color::WHITE)
                .h_align_left()
                .v_align_top();

            let mut coordarr = [0u8; 16];

            use std::io::Write;

            if let Ok(..) = write!(
                &mut coordarr as &mut [u8],
                "{}",
                player.character.position.coords
            ) {
                if let Ok(text) = std::str::from_utf8(&coordarr) {
                    draw.text(&self.data.debug_font, text)
                        .position(5.0, 25.0)
                        .color(Color::WHITE);
                }
            }
        } else {
            self.warper.draw_door(draw, &self.data.tiles, &screen);
        }

        self.data.player.draw(draw, &player.character, color);
        if !player.state.debug_draw {
            self.data.player.bush.draw(draw, &screen);
            self.warper.draw(draw);
        }
    }
}

fn on_tile<P, B: Default>(
    map: &WorldMap,
    player: &PlayerCharacter<P, B>,
    data: &mut ClientWorldData,
    // sender: &Sender<WorldEvent>,
) {
    data.player.bush.check(map, player.character.position.coords);
    // check for wild encounter

    // if let Some(tile_id) = map.tile(player.position.coords) {

    //     // try running scripts

    //     if player.state.scripts.actions.is_empty() {
    //         'scripts: for script in map.scripts.iter() {
    //             use worldlib::script::world::Condition;
    //             for condition in &script.conditions {
    //                 match condition {
    //                     Condition::Location(location) => {
    //                         if !location.in_bounds(&player.position.coords) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                     Condition::Activate(direction) => {
    //                         if player.position.direction.ne(direction) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                     Condition::NoRepeat => {
    //                         if player.state.scripts.executed.contains(&script.identifier) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                     Condition::Script(script, happened) => {
    //                         if player.state.scripts.executed.contains(script).ne(happened) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                     Condition::PlayerHasPokemon(is_true) => {
    //                         if party.is_empty().eq(is_true) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                 }
    //             }
    //             player
    //                 .world
    //                 .scripts
    //                 .actions
    //                 .extend_from_slice(&script.actions);
    //             player
    //                 .world
    //                 .scripts
    //                 .actions
    //                 .push(worldlib::script::world::WorldAction::Finish(
    //                     script.identifier,
    //                 ));
    //             player.state.scripts.actions.reverse();
    //             break;
    //         }
    //     }
    // }
}

// fn get_mut(world: &mut WorldMapManager) -> Option<&mut WorldMap> {
//     match world.data.location.as_ref() {
//         Some(cur) => world.maps.get_mut(cur),
//         None => None,
//     }
// }
