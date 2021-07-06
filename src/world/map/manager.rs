use std::rc::Rc;

use crate::{
    engine::{
        graphics::byte_texture,
        input::{debug_pressed, down, pressed, Control, DebugBind},
        tetra::{graphics::Color, Context},
        util::{Completable, Entity},
    },
    game::{
        battle_glue::BattleEntryRef,
        is_debug, keybind,
        storage::{data, data_mut, player::PlayerSave},
    },
    pokedex::{
        gui::{bag::BagGui, party::PartyGui},
        item::{ItemId, ItemStack, StackSize},
        moves::FieldMoveId,
    },
    state::{MainStates, game::command::CommandResult},
};

use log::{info, warn};

use worldlib::{
    character::{npc::npc_type::NpcType, sprite::SpriteIndexes, MoveType},
    map::{
        manager::{TryMoveResult, WorldMapManager},
        World, WorldMap, WorldTime,
    },
    positions::{Coordinate, Direction, Location, LocationId},
    serialized::SerializedWorld,
};

use crate::world::{
    battle::random_wild_battle,
    gui::{StartMenu, TextWindow, WorldMapGui},
    GameWorld, RenderCoords, WorldTextures,
};

use super::warp::WarpTransition;

const PLAYER_MOVE_TIME: f32 = 0.12;

pub struct WorldManager {
    pub map_manager: WorldMapManager,

    textures: WorldTextures,

    // GUI
    start_menu: StartMenu,
    text_window: TextWindow,
    world_map: WorldMapGui,

    // Warp
    warp_transition: WarpTransition,
    door: Option<Door>,

    // Movement
    first_direction: Direction,
    player_move_accumulator: f32,

    // Rendering
    screen: RenderCoords,
}

pub struct Door {
    pub position: usize,
    pub tile: worldlib::map::TileId,
    pub accumulator: f32,
    pub open: bool,
}

impl Door {
    pub const DOOR_MAX: f32 = 3.99;
}

fn on_start(wm: &mut WorldMapManager, ctx: &mut Context, music: bool) {
    if let Some(map) = get_mut(wm) {
        map.on_start(ctx, music)
    }
}

fn update(
    wm: &mut WorldMapManager,
    ctx: &mut Context,
    delta: f32,
    door_: &mut Option<Door>,
    first_direction: Direction,
    battle: BattleEntryRef,
    text_window: &mut TextWindow,
) {
    if let Some(door) = door_ {
        match door.open {
            true => {
                if door.accumulator < Door::DOOR_MAX {
                    door.accumulator += delta * 6.0;
                    if door.accumulator >= Door::DOOR_MAX {
                        wm.try_move(first_direction, delta); // maybe?
                        door.accumulator = Door::DOOR_MAX;
                    }
                }
            }
            false => {
                if door.accumulator > 0.0 {
                    door.accumulator -= delta * 6.0;
                    if door.accumulator <= 0.0 {
                        *door_ = None;
                    }
                }
            }
        }
    }

    if let Some(map) = match wm.data.current.as_ref() {
        Some(cur) => wm.maps.get_mut(cur),
        None => None,
    } {
        map.update(ctx, delta, &mut wm.data, battle, text_window);
    }
}

fn draw(
    wm: &WorldMapManager,
    ctx: &mut Context,
    door: &Option<Door>,
    textures: &WorldTextures,
    screen: &RenderCoords,
    border: bool,
) {
    if let Some(map) = wm.get() {
        let color = match map.time {
            WorldTime::Day => Color::WHITE,
            WorldTime::Night => Color::rgb(0.6, 0.6, 0.6),
        };

        match &map.chunk {
            Some(chunk) => {
                map.draw(
                    ctx,
                    textures,
                    door,
                    &screen.offset(chunk.coords),
                    border,
                    color,
                );
                for map in chunk.connections.iter().flat_map(|id| wm.maps.get(id)) {
                    if let Some(chunk) = &map.chunk {
                        map.draw(
                            ctx,
                            textures,
                            &None,
                            &screen.offset(chunk.coords),
                            false,
                            color,
                        );
                    }
                }
            }
            None => map.draw(ctx, textures, door, screen, border, color),
        }

        textures.player.draw(ctx, &wm.data.player.character, color);
    }
}

fn get_mut(wm: &mut WorldMapManager) -> Option<&mut WorldMap> {
    match wm.data.current.as_ref() {
        Some(cur) => wm.maps.get_mut(cur),
        None => None,
    }
}

impl WorldManager {
    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            map_manager: WorldMapManager {
                maps: Default::default(),
                data: Default::default(),
            },

            textures: WorldTextures::new(ctx),

            start_menu: StartMenu::new(ctx, party, bag),
            text_window: TextWindow::new(ctx),
            world_map: Default::default(),

            warp_transition: WarpTransition::new(),
            door: None,

            first_direction: Direction::default(),
            screen: RenderCoords::default(),
            // noclip_toggle: false,
            player_move_accumulator: 0.0,
        }
    }

    pub fn load(&mut self, ctx: &mut Context, world: SerializedWorld) {
        self.textures.setup(ctx, world.textures);

        info!("Finished loading textures!");

        let (textures, types): (
            crate::world::map::texture::npc::NpcTextures,
            crate::world::npc::NpcTypes,
        ) = world
            .npc_types
            .into_iter()
            .map(|npc_type| {
                (
                    (
                        npc_type.config.identifier,
                        byte_texture(ctx, &npc_type.texture),
                    ),
                    (
                        npc_type.config.identifier,
                        NpcType {
                            sprite: SpriteIndexes::from_index(npc_type.config.sprite),
                            text_color: npc_type.config.text_color,
                            trainer: npc_type.config.trainer,
                        },
                    ),
                )
            })
            .unzip();

        unsafe {
            crate::world::npc::NPC_TYPES = Some(types);
        }
        self.textures.npcs.set(textures);

        self.world_map.add_locations(world.map_gui_locs);

        self.map_manager = world.manager;
    }

    pub fn load_with_data(&mut self) {
        self.load_player(data());
    }

    pub fn on_start(&mut self, ctx: &mut Context, battle: BattleEntryRef) {
        self.map_start(ctx, true);
        on_tile(&mut self.map_manager, &mut self.textures, battle);
    }

    pub fn map_start(&mut self, ctx: &mut Context, music: bool) {
        on_start(&mut self.map_manager, ctx, music);
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        delta: f32,
        input_lock: bool,
        battle: BattleEntryRef,
        action: &mut Option<MainStates>,
    ) {
        if self.start_menu.alive() {
            self.start_menu.update(ctx, delta, input_lock, action);
        } else if self.world_map.alive() {
            self.world_map.update(ctx);
            if pressed(ctx, Control::A) {
                if let Some(location) = self.world_map.despawn_get() {
                    self.warp_to_location(location);
                }
            }
        } else {
            if !input_lock {
                if pressed(ctx, Control::Start) {
                    self.start_menu.spawn();
                }

                if is_debug() {
                    self.debug_input(ctx)
                }

                if self
                    .map_manager
                    .data
                    .player
                    .character
                    .position
                    .offset
                    .is_zero()
                    && !self.map_manager.data.player.character.is_frozen()
                    && !self.map_manager.data.player.input_frozen
                {
                    if down(ctx, Control::B) {
                        if self.map_manager.data.player.character.move_type == MoveType::Walking {
                            self.map_manager.data.player.character.move_type = MoveType::Running;
                        }
                    } else if self.map_manager.data.player.character.move_type == MoveType::Running
                    {
                        self.map_manager.data.player.character.move_type = MoveType::Walking;
                    }

                    const SURF: FieldMoveId = unsafe { FieldMoveId::new_unchecked(1718777203) };

                    if down(ctx, keybind(self.first_direction)) {
                        if self.player_move_accumulator > PLAYER_MOVE_TIME {
                            if let Some(result) =
                                self.map_manager.try_move(self.first_direction, delta)
                            {
                                match result {
                                    TryMoveResult::MapUpdate => self.map_start(ctx, true),
                                    TryMoveResult::TrySwim => {
                                        for id in data()
                                            .party
                                            .iter()
                                            .map(|pokemon| {
                                                pokemon.moves.iter().flat_map(|instance| {
                                                    &instance.move_ref.field_id
                                                })
                                            })
                                            .flatten()
                                        {
                                            if id == &SURF {
                                                self.map_manager.data.player.character.move_type =
                                                    MoveType::Swimming;
                                                self.map_manager
                                                    .try_move(self.first_direction, delta);
                                                break;
                                            }
                                        }
                                        // self.text_window.set_text(firecore_game::text::Message::new(firecore_game::text::TextColor::Black, vec![]));
                                    }
                                    TryMoveResult::StartWarpOnTile(tile, coords) => {
                                        if self.textures.tiles.has_door(&tile) {
                                            self.door = Some(Door {
                                                position: coords.x as usize
                                                    + coords.y as usize
                                                        * self.map_manager.get().unwrap().width,
                                                tile,
                                                accumulator: 0.0,
                                                open: true,
                                            });
                                        }
                                        self.map_manager.player().input_frozen = true;
                                    }
                                }
                            }
                        } else {
                            self.player_move_accumulator += delta;
                        }
                    } else {
                        let mut movdir: Option<Direction> = None;
                        for direction in &Direction::DIRECTIONS {
                            let direction = *direction;
                            if down(ctx, keybind(direction)) {
                                movdir = if let Some(dir) = movdir {
                                    if dir.inverse() == direction {
                                        None
                                    } else {
                                        Some(direction)
                                    }
                                } else {
                                    Some(direction)
                                };
                            }
                        }
                        if let Some(direction) = movdir {
                            self.map_manager.data.player.character.position.direction = direction;
                            self.first_direction = direction;
                        } else {
                            self.player_move_accumulator = 0.0;
                            self.map_manager.data.player.character.moving = false;
                        }
                    }
                }
            }

            // Update

            self.textures.tiles.update(delta);
            self.textures
                .player
                .update(delta, &mut self.map_manager.data.player.character);

            update(
                &mut self.map_manager,
                ctx,
                delta,
                &mut self.door,
                self.first_direction,
                battle,
                &mut self.text_window,
            );

            if self.warp_transition.alive() {
                self.warp_transition.update(delta);
                if self.warp_transition.switch() {
                    if let Some(destination) = self.map_manager.data.warp {
                        self.textures.player.draw = !destination.transition.move_on_exit;
                        let change_music = destination.transition.change_music;
                        let position = destination.position;
                        if self.map_manager.warp(destination) {
                            let tile = self.map_manager.tile(position.coords).unwrap_or_default();
                            if self.textures.tiles.has_door(&tile) {
                                self.door = Some(Door {
                                    position: position.coords.x as usize
                                        + position.coords.y as usize
                                            * self.map_manager.get().unwrap().width,
                                    tile,
                                    accumulator: 0.0,
                                    open: true,
                                });
                            }
                        }
                        self.map_start(ctx, change_music);
                        on_tile(&mut self.map_manager, &mut self.textures, battle);
                    }
                }
                if self.warp_transition.finished() {
                    self.textures.player.draw = true;
                    self.warp_transition.despawn();
                    self.map_manager.data.player.character.unfreeze();
                    self.map_manager.player().input_frozen = false;
                    if let Some(destination) = self.map_manager.data.warp.take() {
                        if destination.transition.move_on_exit {
                            self.map_manager.try_move(
                                destination.position.direction.unwrap_or(
                                    self.map_manager.data.player.character.position.direction,
                                ),
                                delta,
                            );
                        }
                    }
                }
            } else {
                if let Some(warp) = &self.map_manager.data.warp {
                    if if let Some(door) = &self.door {
                        door.accumulator == 0.0 && !door.open
                    } else {
                        true
                    } || !warp.transition.warp_on_tile
                    {
                        self.warp_transition.spawn();
                        self.map_manager.data.player.input_frozen = true;
                    }
                }
            }

            if !self.map_manager.data.player.character.is_frozen() {
                if if let Some(door) = &self.door {
                    !door.open || door.accumulator == Door::DOOR_MAX
                } else {
                    true
                } {
                    if self.map_manager.data.player.do_move(delta) {
                        self.stop_player(battle);
                    }
                }
            }

            let offset = match self
                .map_manager
                .get()
                .map(|map| map.chunk.as_ref())
                .flatten()
            {
                Some(chunk) => chunk.coords,
                None => Coordinate::ZERO,
            };

            self.screen = RenderCoords::new(offset, &self.map_manager.data.player.character);
        }
    }

    pub fn save_data(&self, save: &mut PlayerSave) {
        save.location = self
            .map_manager
            .data
            .current
            .unwrap_or_else(crate::game::storage::player::default_location);
        save.character = self.map_manager.data.player.character.clone();
    }

    fn stop_player(&mut self, battle: BattleEntryRef) {
        self.map_manager.data.player.character.stop_move();

        if let Some(destination) = self
            .map_manager
            .warp_at(self.map_manager.data.player.character.position.coords)
        {
            // Warping does not trigger tile actions!
            self.map_manager.data.warp = Some(*destination);
        } else if self
            .map_manager
            .in_bounds(self.map_manager.data.player.character.position.coords)
        {
            on_tile(&mut self.map_manager, &mut self.textures, battle);
        }

        if let Some(door) = &mut self.door {
            if self.map_manager.data.warp.is_some() {
                self.textures.player.draw = false;
            }
            door.open = false;
        }
    }

    pub fn load_player(&mut self, data: &PlayerSave) {
        self.map_manager.data.player.character = data.character.clone();
        self.map_manager.data.current = Some(data.location);
    }

    fn debug_input(&mut self, ctx: &Context) {
        if debug_pressed(ctx, DebugBind::F3) {
            info!(
                "Local Coordinates: {}",
                self.map_manager.data.player.character.position.coords
            );

            match self
                .map_manager
                .tile(self.map_manager.data.player.character.position.coords)
            {
                Some(tile) => info!("Current Tile ID: {:x}", tile),
                None => info!("Currently out of bounds"),
            }

            info!(
                "Player is {:?}",
                self.map_manager.data.player.character.move_type
            );
        }

        if debug_pressed(ctx, DebugBind::F5) {
            if let Some(map) = self.map_manager.get() {
                info!("Resetting battled trainers in this map! ({})", map.name);
                data_mut().world.get_map(&map.id).battled.clear();
            }
        }
    }

    fn warp_to_location(&mut self, location: Location) {
        if let Some(map) = self.map_manager.maps.get(&location) {
            info!("Warping to {}", map.name);
            data_mut().location = location;
            self.map_manager.data.current = Some(location);
            let coordinate = if let Some(coord) = map.fly_position {
                coord
            } else if let Some(coord) = map.tenth_walkable_coord() {
                coord
            } else {
                Coordinate::default()
            };

            let pos = &mut self.map_manager.data.player.character.position;
            pos.coords = coordinate;
            pos.direction = Direction::Down;
        }
    }

    pub fn process(&mut self, mut result: CommandResult, battle: BattleEntryRef) {
        match result.command {
            "help" => {
                info!("To - do: help list.");
                info!("To - do: show messages in game")
            }
            "fly_temp" => {
                self.world_map.spawn();
            }
            "heal" => {
                data_mut().party.iter_mut().for_each(|pokemon| {
                    pokemon.heal();
                });
                info!("Healed player pokemon.");
            }
            "wild" => {
                use std::sync::atomic::Ordering::Relaxed;
                let wild = !super::WILD_ENCOUNTERS.load(Relaxed);
                super::WILD_ENCOUNTERS.store(wild, Relaxed);
                info!("Wild Encounters: {}", wild);
            }
            "noclip" => {
                self.map_manager.data.player.character.noclip =
                    !self.map_manager.data.player.character.noclip;
                info!(
                    "Toggled no clip! ({})",
                    self.map_manager.data.player.character.noclip
                );
            }
            "unfreeze" => {
                let player = &mut self.map_manager.data.player;
                player.character.unfreeze();
                player.input_frozen = false;
                info!("Unfroze player!");
            }
            "party" => match result.args.next() {
                Some(arg) => match arg {
                    "info" => match result.args.next() {
                        Some(index) => match index.parse::<usize>() {
                            Ok(index) => {
                                if let Some(instance) = data().party.get(index) {
                                    info!(
                                        "Party Slot {}: Lv{} {}",
                                        index,
                                        instance.level,
                                        instance.name()
                                    );
                                } else {
                                    info!("Could not get pokemon at index {}", index);
                                }
                            }
                            Err(err) => warn!(
                                "Could not parse pokemon index for /party with error {}",
                                err
                            ),
                        },
                        None => {
                            for (slot, instance) in data().party.iter().enumerate() {
                                info!(
                                    "Party Slot {}: Lv{} {}",
                                    slot,
                                    instance.level,
                                    instance.name()
                                );
                            }
                        }
                    },
                    _ => (),
                },
                None => self.start_menu.spawn_party(),
            },
            "battle" => match result.args.next() {
                Some(arg) => match arg {
                    "random" => {
                        match result.args.next() {
                            Some(len) => match len.parse::<usize>() {
                                Ok(size) => random_wild_battle(battle, size),
                                Err(err) => {
                                    warn!("Could not parse battle length for second /battle argument \"{}\" with error {}", len, err);
                                }
                            },
                            None => random_wild_battle(
                                battle,
                                super::super::battle::DEFAULT_RANDOM_BATTLE_SIZE,
                            ),
                        };
                    }
                    _ => warn!("Unknown /battle argument \"{}\".", arg),
                },
                None => warn!("Command /battle requires arguments TODO"),
            },
            "script" => match result.args.next() {
                Some(arg) => match arg {
                    "clear" => {
                        data_mut().world.scripts.clear();
                        info!("Cleared used scripts in player data!");
                    }
                    "list" => {
                        todo!("World script list");
                    }
                    _ => warn!("Unknown /script argument \"{}\".", arg),
                },
                None => warn!("/script requires arguments \"clear\" or \"list\"."),
            },
            "warp" => {
                if let Some(map_or_index) = result
                    .args
                    .next()
                    .map(|a| a.parse::<LocationId>().ok())
                    .flatten()
                {
                    let location = if let Some(index) = result
                        .args
                        .next()
                        .map(|a| a.parse::<LocationId>().ok())
                        .flatten()
                    {
                        Location::new(Some(map_or_index), index)
                    } else {
                        Location::new(None, map_or_index)
                    };
                    self.warp_to_location(location);
                } else {
                    warn!("Invalid warp command syntax!")
                }
            }
            "give" => {
                if let Some(item) = result
                    .args
                    .next()
                    .map(|item| item.parse::<ItemId>().ok())
                    .flatten()
                {
                    let count = result
                        .args
                        .next()
                        .map(|count| count.parse::<StackSize>().ok())
                        .flatten()
                        .unwrap_or(1);
                    data_mut().bag.add_item(ItemStack::new(&item, count));
                }
            }
            _ => warn!("Unknown world command \"{}\".", result),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.start_menu.fullscreen() {
            self.start_menu.draw(ctx);
        } else if self.world_map.alive() {
            self.world_map.draw(ctx);
        } else {
            draw(
                &self.map_manager,
                ctx,
                &self.door,
                &self.textures,
                &self.screen,
                true,
            );

            let offset = match self
                .map_manager
                .get()
                .map(|map| map.chunk.as_ref())
                .flatten()
            {
                Some(chunk) => chunk.coords,
                None => Coordinate::ZERO,
            };

            let screen = self.screen.offset(offset);

            self.textures.player.bush.draw(ctx, &screen);

            self.text_window.draw(ctx);
            self.start_menu.draw(ctx);
            self.warp_transition.draw(ctx);
        }
    }
}

fn on_tile(wm: &mut WorldMapManager, textures: &mut WorldTextures, battle: BattleEntryRef) {
    textures.player.bush.in_bush = wm.tile(wm.data.player.character.position.coords) == Some(0x0D);
    if textures.player.bush.in_bush {
        textures
            .player
            .bush
            .add(wm.data.player.character.position.coords);
    }
    if let Some(map) = match wm.data.current.as_ref() {
        Some(cur) => wm.maps.get_mut(cur),
        None => None,
    } {
        map.on_tile(&mut wm.data, battle)
    }
}
