use std::ops::Deref;

use hashbrown::HashMap;
use rand::{prelude::IteratorRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use pokedex::{
    item::{Item, SavedItemStack},
    moves::Move,
    moves::MoveId,
    pokemon::Pokemon,
    Dex,
};

use text::{MessagePage, MessageState, MessageStates};

use crate::{
    character::{
        action::ActionQueue,
        message::process_str_player,
        npc::{
            group::{NpcGroup, NpcGroupId, TrainerGroup, TrainerGroupId},
            trainer::TrainerDisable,
            Npc, NpcInteract,
        },
        player::{InitPlayerCharacter, PlayerCharacter},
        DoMoveResult, MovementType,
    },
    map::{object::ObjectId, MovementId, WarpDestination, WorldMap},
    positions::{BoundingBox, Coordinate, Direction, Location, Spot},
    script::{WorldInstruction, WorldScriptData},
    state::WorldEvent,
};

use super::{
    battle::BattleEntry,
    chunk::Connection,
    movement::{Elevation, MapMovementResult},
    wild::{WildChances, WildEntry, WildType},
};

use self::{random::WorldRandoms, tile::PaletteDataMap};

pub mod tile;

mod random;

pub type Maps = HashMap<Location, WorldMap>;

pub struct WorldMapManager<R: Rng + SeedableRng + Clone> {
    pub data: WorldMapData,
    pub scripts: WorldScriptData,
    randoms: WorldRandoms<R>,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Move(Direction),
    Interact,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldMapData {
    pub maps: Maps,
    pub palettes: PaletteDataMap,
    pub npc: WorldNpcData,
    pub wild: WildChances,
    pub spawn: Spot,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldNpcData {
    pub groups: HashMap<NpcGroupId, NpcGroup>,
    pub trainers: HashMap<TrainerGroupId, TrainerGroup>,
}

impl<R: Rng + SeedableRng + Clone> WorldMapManager<R> {
    pub fn new(data: WorldMapData, scripts: WorldScriptData) -> Self {
        Self {
            data,
            scripts,
            randoms: Default::default(),
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.randoms.seed(seed);
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.data.maps.contains_key(location)
    }

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.data.maps.get(location)
    }

    pub fn on_warp<P, B: Default>(&mut self, player: &mut PlayerCharacter<P, B>) {
        self.on_map_change(player);
        self.on_tile(player);
    }

    pub fn on_map_change<P, B: Default>(&self, player: &mut PlayerCharacter<P, B>) {
        if let Some(map) = self.data.maps.get(&player.location) {
            self.on_change(map, player);
        }
    }

    pub fn on_change<P, B: Default>(&self, map: &WorldMap, player: &mut PlayerCharacter<P, B>) {
        player.state.events.push(WorldEvent::PlayMusic(map.music));
        // check for cave here and add last spot non cave for escape rope
    }

    pub fn input<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        input: InputEvent,
    ) {
        match input {
            InputEvent::Move(direction) => self.try_move_player(player, direction),
            InputEvent::Interact => player.character.queue_interact(true),
        }
    }

    pub fn try_interact<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        itemdex: &impl Dex<Item, Output = I>,
    ) {
        if let Some(map) = self.data.maps.get_mut(&player.location) {
            let pos = if map
                .tile(player.character.position.coords)
                .map(|tile| {
                    self.data
                        .palettes
                        .get(tile.palette(&map.palettes))
                        .map(|data| (tile.id(), data))
                })
                .flatten()
                .map(|(tile, data)| data.forwarding.contains(&tile))
                .unwrap_or_default()
            {
                player
                    .character
                    .position
                    .in_direction(player.character.position.direction)
            } else {
                player.character.position
            };
            for npc in map.npcs.values_mut() {
                if npc.character.interact_from(&pos) {
                    player.character.input_lock.increment();
                    break;
                }
            }
            let forward = player
                .character
                .position
                .coords
                .in_direction(player.character.position.direction);

            if let Some(object) = map.object_at(&forward) {
                const TREE: &ObjectId =
                    unsafe { &ObjectId::from_bytes_unchecked(1701147252u64.to_ne_bytes()) };
                const CUT: &MoveId = unsafe {
                    &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
                        7632227u128.to_ne_bytes(),
                    ))
                };

                const ROCK: &ObjectId =
                    unsafe { &ObjectId::from_bytes_unchecked(1801678706u64.to_ne_bytes()) };
                /// "rock-smash"
                const ROCK_SMASH: &MoveId = unsafe {
                    &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
                        493254510180952753532786u128.to_ne_bytes(),
                    ))
                };

                fn try_break<
                    P: Deref<Target = Pokemon> + Clone,
                    M: Deref<Target = Move> + Clone,
                    I: Deref<Target = Item> + Clone,
                >(
                    location: &Location,
                    coordinate: Coordinate,
                    id: &MoveId,
                    player: &mut InitPlayerCharacter<P, M, I>,
                ) {
                    if player
                        .trainer
                        .party
                        .iter()
                        .any(|p| p.moves.iter().any(|m| m.id() == id))
                    {
                        player
                            .state
                            .events
                            .push(WorldEvent::BreakObject(coordinate));
                        player.state.insert_object(location, coordinate);
                    }
                }

                if let Some(id) = match &object.group {
                    TREE => Some(CUT),
                    ROCK => Some(ROCK_SMASH),
                    _ => None,
                } {
                    try_break(&map.id, forward, id, player)
                }
            }

            if let Some(item) = map.item_at(&forward) {
                if !player.state.contains_object(&map.id, &forward) {
                    player.state.insert_object(&map.id, forward);
                    let bag = &mut player.trainer.bag;
                    if let Some(item) = item.item.init(itemdex) {
                        bag.insert(item);
                    }
                }
            }
        }
    }

    fn npc_interact<P, B: Default>(
        scripts: &WorldScriptData,
        player: &mut PlayerCharacter<P, B>,
        npc: &mut Npc,
        result: Option<DoMoveResult>,
    ) {
        match result {
            Some(result) => match result {
                DoMoveResult::Finished => (),
                DoMoveResult::Interact => match &npc.interact {
                    NpcInteract::Script(script) => {
                        match player.state.scripts.environment.running() {
                            false => match scripts.scripts.get(script) {
                                Some(instructions) => {
                                    npc.character.on_interact();
                                    let env = &mut player.state.scripts.environment;
                                    env.executor = Some(npc.id);
                                    env.queue = instructions.clone();
                                }
                                None => {
                                    log::warn!(
                                        "Could not get script with id {} for NPC {}",
                                        script,
                                        npc.character.name
                                    );
                                }
                            },
                            true => log::debug!("Could not run script as one is running already!"),
                        }
                    }
                    NpcInteract::Nothing => (),
                },
            },
            None => (),
        }
    }

    pub fn move_npcs<P, B: Default>(&mut self, player: &mut PlayerCharacter<P, B>, delta: f32) {
        if let Some(map) = self.data.maps.get_mut(&player.location) {
            // Move Npcs

            for npc in player
                .state
                .scripts
                .npcs
                .values_mut()
                .filter(|(location, ..)| map.contains(location))
                .map(|(.., npc)| npc)
            {
                npc.character.do_move(delta);
            }

            for npc in map.npcs.values_mut() {
                let r = npc.character.do_move(delta);
                Self::npc_interact(&self.scripts, player, npc, r);
            }

            use crate::character::npc::NpcMovement;

            match player.state.npc.timer > 0.0 {
                false => {
                    player.state.npc.timer += 1.0;

                    const NPC_MOVE_CHANCE: f64 = 1.0 / 12.0;

                    for npc in map.npcs.values_mut() {
                        if !npc.character.moving() {
                            if self.randoms.npc.gen_bool(NPC_MOVE_CHANCE) {
                                for movement in npc.movement.iter() {
                                    match movement {
                                        NpcMovement::Look(directions) => {
                                            if let Some(direction) =
                                                directions.iter().choose(&mut self.randoms.npc)
                                            {
                                                npc.character.position.direction = *direction;
                                            }
                                        }
                                        NpcMovement::Move(area) => {
                                            let origin = npc
                                                .origin
                                                .get_or_insert(npc.character.position.coords);

                                            let next = npc.character.position.forwards();

                                            let bb = BoundingBox::centered(*origin, *area);

                                            if bb.contains(&next)
                                                && next != player.character.position.coords
                                            {
                                                if let Some(code) = map.movements.get(
                                                    npc.character.position.coords.x as usize
                                                        + npc.character.position.coords.y as usize
                                                            * map.width as usize,
                                                ) {
                                                    if Elevation::can_move(
                                                        npc.character.position.elevation,
                                                        *code,
                                                    ) {
                                                        npc.character.actions.queue.push(
                                                            ActionQueue::Move(
                                                                npc.character.position.direction,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                true => player.state.npc.timer -= delta,
            }
        }
    }

    pub fn update_script<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        itemdex: &impl Dex<Item, Output = I>,
    ) {
        let env = &mut player.state.scripts.environment;
        let variables = &mut player.state.scripts.variables;
        let queue = &mut env.queue;

        fn insert(queue: &mut Vec<WorldInstruction>, script: &Vec<WorldInstruction>) {
            match script.contains(&WorldInstruction::End) {
                true => {
                    queue.truncate(1);
                    queue.extend_from_slice(script);
                }
                false => {
                    let mut new = Vec::with_capacity(queue.len() + script.len());
                    new.extend(script.iter().cloned());
                    new.append(queue);
                    std::mem::swap(queue, &mut new);
                }
            };
        }

        if let Some(instruction) = queue.first_mut() {
            match instruction {
                WorldInstruction::End => {
                    if let Some(ref executor) = env.executor.take() {
                        if let Some(npc) = self
                            .data
                            .maps
                            .get_mut(&player.location)
                            .map(|map| map.npcs.get_mut(executor).map(|npc| npc))
                            .flatten()
                        {
                            npc.character.end_interact()
                        }
                    }
                    queue.remove(0);
                }
                WorldInstruction::Lock => {
                    player.character.locked.increment();
                    queue.remove(0);
                }
                WorldInstruction::Release => {
                    player.character.locked.decrement();
                    queue.remove(0);
                }
                WorldInstruction::SetVar(id, var) => {
                    variables.insert(id.clone(), *var);
                    queue.remove(0);
                }
                WorldInstruction::SpecialVar(name, func) => {
                    match func.as_str() {
                        "ShouldTryRematchBattle" => {
                            // to - do: rematches
                            variables.insert(
                                name.clone(),
                                match player.state.scripts.flags.contains("REMATCHES") {
                                    true => 1,
                                    false => 0,
                                },
                            );
                        }
                        _ => (),
                    }
                    queue.remove(0);
                }
                WorldInstruction::Compare(name, var) => {
                    if variables.get(name) == Some(var) {
                        player.state.scripts.flags.insert("TEMP_EQ".to_owned());
                    }
                    queue.remove(0);
                }
                WorldInstruction::GotoIfEq(script) => {
                    if player.state.scripts.flags.remove("TEMP_EQ") {
                        match self.scripts.scripts.get(script) {
                            Some(script) => insert(queue, script),
                            None => log::warn!(
                                "Could not get script {} for GotoIfEq instruction",
                                script,
                            ),
                        }
                    } else {
                        queue.remove(0);
                    }
                }
                WorldInstruction::GotoIfSet(id, script) => {
                    if variables.contains_key(id) {
                        match self.scripts.scripts.get(script) {
                            Some(script) => insert(queue, script),
                            None => log::warn!(
                                "Could not get script {} for GotoIfSet instruction querying {}",
                                script,
                                id
                            ),
                        }
                    }
                    queue.remove(0);
                }
                WorldInstruction::Return => {
                    if queue.len() == 1 {
                        queue.push(WorldInstruction::End);
                    }
                    queue.remove(0);
                }

                WorldInstruction::SetFlag(flag) => {
                    player.state.scripts.flags.insert(flag.clone());
                    queue.remove(0);
                }
                WorldInstruction::Call(script) => {
                    match self.scripts.scripts.get(script) {
                        Some(script) => insert(queue, script),
                        None => log::warn!("Could not get script {} for Call instruction", script,),
                    }
                    queue.remove(0);
                }
                WorldInstruction::TextColor(color) => {
                    variables.insert("TEMP_TEXTCOLOR".to_owned(), *color as _);
                    queue.remove(0);
                }
                WorldInstruction::Message(..) => {
                    if let WorldInstruction::Message(id) =
                        std::mem::replace(instruction, WorldInstruction::End)
                    {
                        let color = variables.remove("TEMP_TEXTCOLOR").map(|n| match n {
                            _ => String::new(),
                        });
                        queue[0] = WorldInstruction::Msgbox(id, color);
                    }
                }
                WorldInstruction::WaitMessage => {
                    log::warn!("Add WaitMessage instruction!");
                    queue.remove(0);
                }
                WorldInstruction::PlayFanfare(id, variant) => {
                    player
                        .state
                        .events
                        .push(WorldEvent::PlaySound(*id, *variant));
                    queue.remove(0);
                }
                WorldInstruction::WaitFanfare() => {
                    log::warn!("Add WaitFanfare instruction!");
                    queue.remove(0);
                }
                WorldInstruction::AddItem(item) => {
                    if let Some(stack) = SavedItemStack::from(*item).init(itemdex) {
                        player.trainer.bag.insert(stack);
                    }
                    queue.remove(0);
                }
                WorldInstruction::CheckItemSpace(..) => {
                    log::warn!("Add CheckItemSpace instruction!");
                    queue.remove(0);
                }
                WorldInstruction::GetItemName(..) => {
                    log::warn!("Add GetItemName instruction!");
                    queue.remove(0);
                }
                // WorldInstruction::Walk(..) | WorldInstruction::FacePlayer | WorldInstruction::TrainerBattleSingle | WorldInstruction::Msgbox(..) => {
                npc_inst => {
                    if let Some((map, settings, npc)) = env
                        .executor
                        .as_ref()
                        .map(|id| {
                            self.data
                                .maps
                                .get_mut(&player.location)
                                .map(|map| {
                                    map.npcs
                                        .get_mut(id)
                                        .map(|npc| (&map.id, &map.settings, npc))
                                })
                                .flatten()
                        })
                        .flatten()
                    {
                        match npc_inst {
                            WorldInstruction::TrainerBattleSingle => {
                                match player.state.scripts.flags.contains("TEMP_MESSAGE") {
                                    true => {
                                        if !player.state.message.is_running() {
                                            player.state.scripts.flags.remove("TEMP_MESSAGE");
                                            if let Some(entry) = BattleEntry::trainer(
                                                &mut player.state.battle,
                                                map,
                                                &settings,
                                                &self.data.npc,
                                                &npc.id,
                                                npc,
                                            ) {
                                                player.state.battle.battling = Some(entry);
                                            }
                                            //     let this = &mut queue[0];
                                            //     *this = WorldInstruction::End;
                                            //     self.update_script(player);
                                            queue.remove(0);
                                        }
                                    }
                                    false => {
                                        if player.trainer.party.is_empty()
                                            || player.state.battle.battled(map, &npc.id)
                                        {
                                            queue.remove(0);
                                        } else if let Some(trainer) = npc.trainer.as_ref() {
                                            drop(variables);
                                            player
                                                .state
                                                .scripts
                                                .flags
                                                .insert("TEMP_MESSAGE".to_owned());
                                            let message = MessageState {
                                                pages: trainer
                                                    .encounter
                                                    .iter()
                                                    .map(|lines| MessagePage {
                                                        lines: lines
                                                            .iter()
                                                            .map(|str| {
                                                                process_str_player(str, player)
                                                            })
                                                            .collect(),
                                                        wait: None,
                                                        color: self
                                                            .data
                                                            .npc
                                                            .groups
                                                            .get(&npc.group)
                                                            .map(|g| g.message),
                                                    })
                                                    .collect::<Vec<_>>(),
                                                ..Default::default()
                                            };
                                            player.state.message = MessageStates::Running(message);
                                        }
                                    }
                                }
                            }
                            WorldInstruction::Msgbox(m_id, msgbox_type) => {
                                if !player.state.message.is_running() {
                                    match player.state.scripts.flags.contains("TEMP_MESSAGE") {
                                        true => {
                                            player.state.scripts.flags.remove("TEMP_MESSAGE");
                                            queue.remove(0);
                                        }
                                        false => match self.scripts.messages.get(m_id) {
                                            Some(message) => {
                                                let message = MessageState {
                                                    pages: message
                                                        .iter()
                                                        .map(|lines| MessagePage {
                                                            lines: lines
                                                                .iter()
                                                                .map(|str| {
                                                                    process_str_player(str, player)
                                                                })
                                                                .collect(),
                                                            wait: None,
                                                            color: self
                                                                .data
                                                                .npc
                                                                .groups
                                                                .get(&npc.group)
                                                                .map(|g| g.message),
                                                        })
                                                        .collect::<Vec<_>>(),
                                                    ..Default::default()
                                                };
                                                player.state.message =
                                                    MessageStates::Running(message);
                                                player
                                                    .state
                                                    .scripts
                                                    .flags
                                                    .insert("TEMP_MESSAGE".to_owned());
                                            }
                                            None => {
                                                queue.remove(0);
                                            }
                                        },
                                    }
                                }
                            }
                            WorldInstruction::FacePlayer => {
                                let pos = &mut npc.character.position;
                                pos.direction =
                                    pos.coords.towards(player.character.position.coords);
                                queue.remove(0);
                            }
                            WorldInstruction::Walk(direction) => {
                                if !npc.character.moving() {
                                    match player.state.scripts.flags.contains("TEMP_0") {
                                        true => {
                                            player.state.scripts.flags.remove("TEMP_0");
                                            queue.remove(0);
                                        }
                                        false => {
                                            npc.character
                                                .actions
                                                .queue
                                                .push(ActionQueue::Move(*direction));
                                            player.state.scripts.flags.insert("TEMP_0".to_owned());
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        } else if env.executor.is_some() {
            env.executor = None;
        }
    }

    pub fn on_tile<P, B: Default>(
        &mut self,
        player: &mut PlayerCharacter<P, B>,
        // party: &[OwnedPokemon<P, M, I>],
    ) {
        player.state.events.push(WorldEvent::OnTile);

        if !player.trainer.party.is_empty() {
            if let Some(map) = self.data.maps.get_mut(&player.location) {
                if player.state.wild.encounters {
                    if let Some(current) = map.tile(player.character.position.coords) {
                        if self
                            .data
                            .palettes
                            .get(current.palette(&map.palettes))
                            .map(|data| data.wild.contains(&current.id()))
                            .unwrap_or_default()
                        {
                            let t = match &player.character.movement {
                                MovementType::Swimming => &WildType::Water,
                                _ => &WildType::Land,
                            };
                            if let Some(entry) =
                                &map.wild.as_ref().map(|entries| entries.get(t)).flatten()
                            {
                                if let Some(entry) = WildEntry::generate(
                                    &self.data.wild,
                                    t,
                                    entry,
                                    &mut self.randoms.wild,
                                ) {
                                    player.state.battle.battling = Some(entry);
                                }
                            }
                        }
                    }
                }

                for npc in map.npcs.values_mut() {
                    if let Some(trainer) = &npc.trainer {
                        player.find_battle(&map.id, &npc.id, trainer, &mut npc.character);
                    }
                }
            }
        }
    }

    fn stop_player<P, B: Default>(&mut self, player: &mut PlayerCharacter<P, B>) {
        player.character.stop_move();

        if let Some(map) = self.data.maps.get(&player.location) {
            if let Some(destination) = map.warp_at(&player.character.position.coords) {
                // Warping does not trigger tile actions!
                player.state.warp = Some(*destination);
            } else if map.in_bounds(player.character.position.coords) {
                self.on_tile(player);
            }
        }
    }

    pub fn update<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        itemdex: &impl Dex<Item, Output = I>,
        delta: f32,
    ) {
        if let Some(result) = player.update(delta) {
            match result {
                DoMoveResult::Finished => self.stop_player(player),
                DoMoveResult::Interact => self.try_interact(player, itemdex),
            }
        }
        self.move_npcs(player, delta);
        self.update_script(player, itemdex);
        // self.update_interactions(player);
    }

    pub fn try_move_player<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        player: &mut InitPlayerCharacter<P, M, I>,
        direction: Direction,
    ) {
        player.character.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = player.character.position.coords + offset;

        if let Some(map) = self.get(&player.location) {
            // Check for warp on tile
            if player.state.warp.is_none() {
                if let Some(destination) = map.warp_at(&coords) {
                    player.state.warp = Some(*destination);
                    player
                        .state
                        .events
                        .push(WorldEvent::BeginWarpTransition(coords));
                    return;
                }
            };

            // Check for one-way tile
            if map
                .tile(coords)
                .map(|tile| {
                    self.data
                        .palettes
                        .get(tile.palette(&map.palettes))
                        .map(|data| {
                            data.cliffs
                                .get(&direction)
                                .map(|tiles| tiles.contains(&tile.id()))
                        })
                })
                .flatten()
                .flatten()
                .unwrap_or_default()
                && !player.character.noclip
            {
                player.state.events.push(WorldEvent::PlayerJump);
                return;
            }

            match map.chunk_movement(coords, player) {
                MapMovementResult::Option(code) => {
                    with_code(player, code.unwrap_or(1), direction);
                }
                MapMovementResult::Chunk(direction, offset, connection) => {
                    if let Some((location, coords, code)) = self
                        .data
                        .connection_movement(direction, offset, connection, player)
                    {
                        if with_code(player, code, direction) {
                            player.character.position.coords = coords;
                            player.location = location;
                            self.on_map_change(player);
                        }
                    }
                }
            }
        }

        fn with_code<
            P: Deref<Target = Pokemon> + Clone,
            M: Deref<Target = Move> + Clone,
            I: Deref<Target = Item> + Clone,
        >(
            player: &mut InitPlayerCharacter<P, M, I>,
            code: MovementId,
            direction: Direction,
        ) -> bool {
            if Elevation::can_move(player.character.position.elevation, code)
                || player.character.noclip
            {
                if Elevation::WATER == code {
                    if player.character.movement != MovementType::Swimming {
                        const SURF: &MoveId = unsafe {
                            &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
                                1718777203u128.to_ne_bytes(),
                            ))
                        };

                        if player
                            .trainer
                            .party
                            .iter()
                            .flat_map(|pokemon| pokemon.moves.iter())
                            .any(|m| m.id() == SURF)
                            || player.character.noclip
                        {
                            player.character.movement = MovementType::Swimming;
                        } else {
                            return false;
                        }
                    }
                } else if player.character.movement == MovementType::Swimming {
                    player.character.movement = MovementType::Walking;
                }
                Elevation::change(&mut player.character.position.elevation, code);
                player
                    .character
                    .actions
                    .queue
                    .push(ActionQueue::Move(direction));
                return true;
                // self.player.offset =
                //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
            }
            false
        }
    }

    pub fn warp<P, B: Default>(
        &mut self,
        player: &mut PlayerCharacter<P, B>,
        destination: WarpDestination,
    ) {
        if self.data.warp(player, destination) {
            self.on_warp(player);
        }
    }
}

impl WorldMapData {
    pub fn connection_movement<P, B: Default>(
        &self,
        direction: Direction,
        offset: i32,
        connections: &[Connection],
        player: &PlayerCharacter<P, B>,
    ) -> Option<(Location, Coordinate, MovementId)> {
        connections.iter().find_map(|connection| {
            self.maps
                .get(&connection.0)
                .map(|map| {
                    let o = offset - connection.1;
                    let position = Connection::offset(direction, map, o);
                    let coords = position.in_direction(direction);
                    map.local_movement(coords, player)
                        .map(|code| (map.id, position, code))
                })
                .flatten()
        })
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
        player.character.locked.decrement();
        let entry = player.state.battle.battling.take();
        if winner {
            if let Some(entry) = entry {
                if let Some(trainer) = entry.trainer {
                    if let Some(npc) = self
                        .maps
                        .get_mut(&trainer.location)
                        .map(|map| map.npcs.get_mut(&trainer.id))
                        .flatten()
                    {
                        player.character.end_interact();
                        npc.character.end_interact();
                        if let Some(npc) = &npc.trainer {
                            match &npc.disable {
                                TrainerDisable::DisableSelf => {
                                    player.state.battle.insert(&trainer.location, trainer.id);
                                }
                                TrainerDisable::Many(others) => {
                                    player.state.battle.insert(&trainer.location, trainer.id);
                                    player
                                        .state
                                        .battle
                                        .battled
                                        .get_mut(&trainer.location)
                                        .unwrap()
                                        .extend(others);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let Spot { location, position } = player.state.places.heal.unwrap_or(self.spawn);
            player.location = location;
            player.character.position = position;
            player.location = player.location;
            player
                .trainer
                .party
                .iter_mut()
                .for_each(|o| o.heal(None, None));
        }
    }

    pub fn warp<P, B: Default>(
        &self,
        player: &mut PlayerCharacter<P, B>,
        destination: WarpDestination,
    ) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                player.warp(destination);
                true
            }
            false => false,
        }
    }
}
