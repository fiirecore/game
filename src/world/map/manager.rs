use std::ops::Deref;

use crate::{
    engine::{
        audio,
        error::ImageError,
        graphics::{self, Color},
        input::controls::{pressed, Control},
        text::MessagePage,
        utils::{Completable, Entity},
        Context,
    },
    game::battle_glue::{BattleEntry as GameBattleEntry, BattleId, BattleTrainerEntry},
    saves::{GameBag, GameParty},
    state::game::GameActions,
};

use battlelib::pokedex::{
    item::Item,
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
    Initializable,
};

use firecore_world::{
    actions::WorldActions,
    events::{split, Receiver, Sender},
    map::manager::state::default_heal_loc,
};

use rand::prelude::SmallRng;

use worldlib::{
    character::player::PlayerCharacter,
    map::{chunk::Connection, manager::WorldMapManager, Brightness, WorldMap},
    positions::{Coordinate, Direction, Location},
    serialized::SerializedWorld,
};

use crate::world::{
    gui::TextWindow,
    map::{data::ClientWorldData, input::PlayerInput, warp::WarpTransition},
    npc::color,
    RenderCoords,
};

// pub mod script;

pub struct GameWorldMapManager {
    world: WorldMapManager<SmallRng>,

    data: ClientWorldData,

    warper: WarpTransition,
    text: TextWindow,
    input: PlayerInput,
    // screen: RenderCoords,
    sender: Sender<GameActions>,
    sender2: Sender<WorldActions>,
    receiver: Receiver<WorldActions>,
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
            world: WorldMapManager::new(world.maps, default_heal_loc(), sender.clone()),

            data: ClientWorldData::new(
                ctx,
                world.textures.palettes,
                world.textures.animated,
                world.npc_types,
                world.textures.player,
            )?,
            warper: WarpTransition::new(ctx, world.textures.doors),
            text: TextWindow::new(ctx)?,
            input: PlayerInput::default(),
            sender: actions,
            sender2: sender,
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

    pub fn post_battle<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        player: &mut PlayerCharacter,
        party: &mut [OwnedPokemon<P, M, I>],
        winner: bool,
        trainer: bool,
    ) {
        self.world.post_battle(player, party, winner, trainer)
    }

    pub fn try_warp(&mut self, player: &mut PlayerCharacter, location: Location) -> bool {
        if self.world.contains(&location) {
            self.warp_to_location(player, location);
            true
        } else {
            false
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        player: &mut PlayerCharacter,
        party: &mut GameParty,
        bag: &mut GameBag,
        delta: f32,
    ) {
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }
        // Input

        for action in self.receiver.try_iter() {
            match action {
                WorldActions::Battle(entry) => {
                    let (id, t) = if let Some(trainer) = entry.trainer {
                        let (id, t) = (
                            BattleId::Trainer(trainer.id),
                            if let Some(npc) = self
                                .world
                                .get(&trainer.location)
                                .map(|map| map.npcs.get(&trainer.id))
                                .flatten()
                            {
                                let trainer = npc.trainer.as_ref().unwrap();
                                let npc_type = self.data.npc.types.get(&npc.type_id);
                                Some(BattleTrainerEntry {
                                    name: npc_type
                                        .trainer
                                        .as_ref()
                                        .map(|t| format!("{} {}", t.name, npc.character.name))
                                        .unwrap_or_else(|| npc.character.name.clone()),
                                    badge: trainer.badge,
                                    sprite: npc.type_id,
                                    transition: trainer.battle_transition,
                                    victory_message: trainer.victory_message.clone(),
                                    worth: trainer.worth as _,
                                })
                            } else {
                                None
                            },
                        );
                        player.world.battle.battling = Some(trainer);
                        (id, t)
                    } else {
                        (BattleId::Wild, None)
                    };
                    self.sender.send(GameActions::Battle(GameBattleEntry {
                        id,
                        party: entry.party,
                        trainer: t,
                        active: entry.active,
                    }))
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
                WorldActions::GiveItem(stack) => {
                    if let Some(stack) = stack.init(crate::dex::itemdex()) {
                        bag.insert(stack);
                    }
                }
                // WorldActions::HealPokemon(index) => match index {
                //     Some(index) => {
                //         if let Some(pokemon) = party.get_mut(index) {
                //             pokemon.heal(None, None);
                //         }
                //     }
                //     None => {
                //         for pokemon in party.iter_mut() {
                //             pokemon.heal(None, None);
                //         }
                //     }
                // },
                // WorldActions::Warp(location) => {
                //     if !self.try_warp(player, party, location) {
                //         self.sender
                //             .send(GameActions::CommandError("Unknown location!"));
                //     }
                // }
                WorldActions::Message(npc, pages, queue) => {
                    if !self.text.text.alive() {
                        self.text.text.spawn();
                    }
                    if !queue {
                        self.text.text.pages.clear();
                    }
                    let npc = npc
                        .map(|(npc, music)| {
                            self.world.get(&player.location).map(|map| {
                                map.npcs.get(&npc).map(|npc| {
                                    if music {
                                        if let Some(music) = self
                                            .data
                                            .npc
                                            .types
                                            .get(&npc.type_id)
                                            .trainer
                                            .as_ref()
                                            .map(|trainer| trainer.music)
                                            .flatten()
                                        {
                                            self.sender2.send(WorldActions::PlayMusic(music));
                                        }
                                    }
                                    (
                                        &npc.character,
                                        color(&self.data.npc.types.get(&npc.type_id).message),
                                    )
                                })
                            })
                        })
                        .flatten()
                        .flatten();

                    let mut color = None;

                    let npc = npc.map(|(ch, co)| {
                        color = Some(co);
                        ch
                    });

                    let mut pages = pages
                        .into_iter()
                        .map(|lines| MessagePage {
                            lines,
                            wait: None,
                            color: color.unwrap_or(MessagePage::BLACK),
                        })
                        .collect::<Vec<_>>();

                    crate::game::text::process_messages(&mut pages, player, npc);
                    self.text.text.pages.extend(pages);
                    player.input_frozen = true;
                }
                WorldActions::PlayMusic(music) => {
                    if let Some(playing) = audio::get_current_music(ctx) {
                        if playing != &music {
                            audio::play_music(ctx, &music);
                        }
                    } else {
                        audio::play_music(ctx, &music);
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
            }
        }

        if self.text.text.alive() {
            self.text.text.update(ctx, delta);
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

        if let Some(direction) = self.input.update(player, ctx, delta) {
            self.world.try_move(player, party, direction);
        }

        self.world.update(player, delta);

        if pressed(ctx, Control::A) {
            self.world.try_interact(player);
        }

        self.world
            .update_interactions(player, self.text.text.alive(), self.text.text.finished());

        if self.text.text.finished() {
            player.input_frozen = false;
            self.text.text.despawn();
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

    pub fn warp_to_location(&mut self, player: &mut PlayerCharacter, location: Location) {
        if let Some(map) = self.world.maps.get(&location) {
            player.location = location;
            let pos = &mut player.position;
            pos.coords = map.settings.fly_position.unwrap_or_default();
            pos.direction = Direction::Down;
            self.world.on_warp(player);
        }
    }

    pub fn draw(&self, ctx: &mut Context, player: &PlayerCharacter) {
        let screen = RenderCoords::new(player);

        let color = match self.world.get(&player.location) {
            Some(current) => {
                let color = match current.settings.brightness {
                    Brightness::Day => Color::WHITE,
                    Brightness::Night => Color::rgb(0.6, 0.6, 0.6),
                };

                super::draw(
                    ctx,
                    current,
                    &player.world,
                    &self.data,
                    &screen,
                    true,
                    color,
                );

                match &current.chunk {
                    Some(chunk) => {
                        for (connection, direction, offset) in chunk.connections.iter().flat_map(
                            |(direction, Connection(location, offset))| {
                                self.world
                                    .maps
                                    .get(location)
                                    .map(|map| (map, direction, offset))
                            },
                        ) {
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
                graphics::draw_text_left(ctx, &0, "Cannot get map:", 0.0, 0.0, Default::default());
                graphics::draw_text_left(
                    ctx,
                    &0,
                    player.location.map.as_deref().unwrap_or("None"),
                    0.0,
                    8.0,
                    Default::default(),
                );
                graphics::draw_text_left(
                    ctx,
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
                &1,
                player.location.index.as_str(),
                5.0,
                15.0,
                Default::default(),
            );

            let coordinates = format!("{}", player.character.position.coords);
            graphics::draw_text_left(ctx, &1, &coordinates, 5.0, 25.0, Default::default());
        }

        self.warper.draw_door(ctx, &screen);
        self.data.player.draw(ctx, player, color);
        self.data.player.bush.draw(ctx, &screen);
        self.warper.draw(ctx);
        self.text.draw(ctx);
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
