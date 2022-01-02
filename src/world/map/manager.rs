use crate::{
    engine::{
        audio,
        error::ImageError,
        graphics::{self, Color},
        controls::{pressed, Control},
        text::MessagePage,
        utils::{Completable, Entity},
        Context, EngineContext,
    },
    game::battle_glue::{BattleEntry as GameBattleEntry, BattleId, BattleTrainerEntry},
    state::game::GameActions,
};

use rand::prelude::SmallRng;

use worldlib::{
    actions::{WorldAction, WorldActions},
    character::player::PlayerCharacter,
    events::{split, InputEvent, Receiver, Sender},
    map::{chunk::Connection, manager::WorldMapManager, Brightness, WorldMap},
    positions::{Coordinate, Direction, Location, Position},
    serialized::SerializedWorld,
};

use crate::world::{
    gui::TextWindow,
    map::{data::ClientWorldData, input::PlayerInput, warp::WarpTransition},
    RenderCoords,
};

mod npc;

// pub mod script;

pub struct GameWorldMapManager {
    world: WorldMapManager<SmallRng>,

    data: ClientWorldData,

    warper: WarpTransition,
    text: TextWindow,
    input: PlayerInput,
    // screen: RenderCoords,
    sender: Sender<GameActions>,
    receiver: Receiver<WorldAction>,
    // events: EventReceiver<WorldEvents>,
}

impl GameWorldMapManager {
    pub(crate) fn new(
        ctx: &mut Context,
        actions: Sender<GameActions>,
        world: SerializedWorld,
    ) -> Result<Self, ImageError> {
        // let events = Default::default();

        let (sender, receiver) = split();

        Ok(Self {
            world: WorldMapManager::new(world.data, sender),

            data: ClientWorldData::new(ctx, world.textures)?,
            warper: WarpTransition::new(),
            text: TextWindow::new(ctx)?,
            input: PlayerInput::default(),
            sender: actions,
            // screen: RenderCoords::default(),
            // events,
            receiver,
        })
    }

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.world.get(location)
    }

    pub fn start(&mut self, player: &mut PlayerCharacter) {
        self.world.on_warp(player);
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
    }

    pub fn post_battle(&mut self, player: &mut PlayerCharacter, winner: bool) {
        self.world.post_battle(player, winner)
    }

    pub fn spawn(&self) -> (Location, Position) {
        self.world.data.spawn
    }

    pub fn try_warp(&mut self, player: &mut PlayerCharacter, location: Location) -> bool {
        if self.world.contains(&location) {
            self.warp_to_location(player, location);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, player: &mut PlayerCharacter, delta: f32) {
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }
        // Input

        for action in self.receiver.try_iter() {
            match action.action {
                WorldActions::Battle(entry) => {
                    if !player.trainer.party.is_empty() {
                        player.freeze();
                        let active = entry.active;
                        let party = entry.party.clone();
                        let (id, t) = if let Some(trainer) = entry.trainer.as_ref() {
                            let (id, t) = (
                                BattleId::Trainer(trainer.id),
                                if let Some((map, npc)) = self
                                    .world
                                    .get(&trainer.location)
                                    .map(|map| map.npcs.get(&trainer.id).map(|npc| (map, npc)))
                                    .flatten()
                                {
                                    let trainer = npc.trainer.as_ref().unwrap();
                                    let group = npc::group(&self.world.data.npcs, &npc.group);
                                    Some(BattleTrainerEntry {
                                        name: group
                                            .trainer
                                            .as_ref()
                                            .map(|t| format!("{} {}", t.name, npc.character.name))
                                            .unwrap_or_else(|| npc.character.name.clone()),
                                        bag: trainer.bag.clone(),
                                        badge: trainer.badge,
                                        sprite: npc.group,
                                        transition: map.settings.transition,
                                        defeat: trainer
                                            .defeat
                                            .iter()
                                            .map(|lines| MessagePage {
                                                lines: lines.clone(),
                                                wait: None,
                                                color: npc::color(&group.message),
                                            })
                                            .collect(),
                                        worth: trainer.worth as _,
                                    })
                                } else {
                                    None
                                },
                            );
                            player.world.battle.battling = Some(entry);
                            (id, t)
                        } else {
                            (BattleId::Wild, None)
                        };
                        self.sender.send(GameActions::Battle(GameBattleEntry {
                            id,
                            party,
                            trainer: t,
                            active,
                        }))
                    };
                }
                WorldActions::PlayerJump => self.data.player.jump(player),
                // WorldActions::GivePokemon(pokemon) => {
                //     if let Some(pokemon) = pokemon.init(
                //         &mut self.data.randoms.general,
                //         crate::pokedex(),
                //         crate::movedex(),
                //         crate::itemdex(),
                //     ) {
                //         party.try_push(pokemon);
                //     }
                // }
                WorldActions::Message(pages, color) => {
                    if !self.text.text.alive() {
                        self.text.text.spawn();
                    }
                    self.text.text.pages.clear();

                    let pages = pages
                        .into_iter()
                        .map(|lines| MessagePage {
                            lines,
                            wait: None,
                            color: npc::color(&color),
                        })
                        .collect::<Vec<_>>();

                    self.text.text.pages.extend(pages);
                    player.input_frozen = true;
                }
                WorldActions::PlayMusic(music) => {
                    if let Some(playing) = audio::get_current_music(eng) {
                        if playing != &music {
                            audio::play_music(ctx, eng, &music);
                        }
                    } else {
                        audio::play_music(ctx, eng, &music);
                    }
                }
                WorldActions::BeginWarpTransition(coords) => {
                    if let Some(map) = self.world.get(&player.location) {
                        if let Some(tile) = map.tile(coords) {
                            self.warper.queue(player, tile, coords);
                        }
                    }
                }
                WorldActions::OnTile => {
                    if let Some(map) = self.world.get(&player.location) {
                        on_tile(map, player, &mut self.data)
                    }
                }
                WorldActions::BreakObject(coordinate) => if let Some(map) = self.world.get(&player.location) {
                    if let Some(object) = map.object_at(&coordinate) {
                        self.data.object.add(coordinate, &object.group);
                    }
                },
            }
        }

        if self.text.text.alive() {
            if self.text.text.finished() {
                if let Some(polling) = &player.world.polling {
                    polling.update()
                }
                player.input_frozen = false;
                self.text.text.despawn();
            }
            self.text.text.update(ctx, eng, delta);
        }

        self.data.update(delta, player);

        if self.warper.alive() {
            if let Some(_music) = self.warper.update(&mut self.world, player, delta) {
                self.world.on_warp(player);
            }
        } else if player.world.warp.is_some() {
            self.warper.spawn();
            player.input_frozen = true;
        }

        if let Some(direction) = self.input.update(ctx, eng, player, delta) {
            self.world.input(player, InputEvent::Move(direction));
        }

        if pressed(ctx, eng, Control::A) {
            self.world.try_interact(player);
        }

        self.world.update(player, delta);
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

    pub fn warp_to_location(&mut self, player: &mut PlayerCharacter, location: Location) {
        if let Some(map) = self.world.data.maps.get(&location) {
            player.location = location;
            let pos = &mut player.position;
            pos.coords = map.settings.fly_position.unwrap_or_default();
            pos.direction = Direction::Down;
            self.world.on_warp(player);
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext, player: &PlayerCharacter) {
        let screen = RenderCoords::new(player);

        let color = match self.world.get(&player.location) {
            Some(current) => {
                let color = match current.settings.brightness {
                    Brightness::Day => Color::WHITE,
                    Brightness::Night => Color::rgb(0.6, 0.6, 0.6),
                };

                super::draw(
                    ctx,
                    eng,
                    current,
                    &player.world,
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
                            ) -> Coordinate {
                                match direction {
                                    Direction::Down => Coordinate::new(offset, current.height),
                                    Direction::Up => Coordinate::new(offset, -map.height),
                                    Direction::Left => Coordinate::new(-map.width, offset),
                                    Direction::Right => Coordinate::new(current.width, offset),
                                }
                            }

                            super::draw(
                                ctx,
                                eng,
                                connection,
                                &player.world,
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
                graphics::draw_text_left(ctx, eng, &0, "Cannot get map:", 0.0, 0.0, Default::default());
                graphics::draw_text_left(
                    ctx,
                    eng,
                    &0,
                    player.location.map.as_deref().unwrap_or("None"),
                    0.0,
                    8.0,
                    Default::default(),
                );
                graphics::draw_text_left(
                    ctx,
                    eng,
                    &0,
                    player.location.index.as_str(),
                    0.0,
                    16.0,
                    Default::default(),
                );
                Color::WHITE
            }
        };

        if player.world.debug_draw {
            graphics::draw_text_left(
                ctx,
                eng,
                &1,
                player
                    .location
                    .map
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("No Base Map ID"),
                5.0,
                5.0,
                Default::default(),
            );
            graphics::draw_text_left(
                ctx,
                eng,
                &1,
                player.location.index.as_str(),
                5.0,
                15.0,
                Default::default(),
            );

            let coordinates = format!("{}", player.character.position.coords);
            graphics::draw_text_left(ctx, eng, &1, &coordinates, 5.0, 25.0, Default::default());
        }

        self.warper.draw_door(ctx, &screen);
        self.data.player.draw(ctx, player, color);
        self.data.player.bush.draw(ctx, &screen);
        self.warper.draw(ctx);
        self.text.draw(ctx, eng);
    }
}

fn on_tile(
    map: &WorldMap,
    player: &PlayerCharacter,
    data: &mut ClientWorldData,
    // sender: &Sender<WorldActions>,
) {
    data.player.bush.check(map, player.position.coords);
    // check for wild encounter

    // if let Some(tile_id) = map.tile(player.position.coords) {

    //     // try running scripts

    //     if player.world.scripts.actions.is_empty() {
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
    //                         if player.world.scripts.executed.contains(&script.identifier) {
    //                             continue 'scripts;
    //                         }
    //                     }
    //                     Condition::Script(script, happened) => {
    //                         if player.world.scripts.executed.contains(script).ne(happened) {
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
    //             player.world.scripts.actions.reverse();
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
