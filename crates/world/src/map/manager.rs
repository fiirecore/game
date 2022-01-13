use firecore_text::{MessagePage, MessageState};
use hashbrown::HashMap;
use rand::{prelude::IteratorRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use pokedex::moves::MoveId;

use crate::{
    actions::WorldActions,
    character::{
        action::ActionQueue,
        message::process_str_player,
        npc::{
            group::{MessageColor, NpcGroup, NpcGroupId, TrainerGroup, TrainerGroupId},
            trainer::TrainerDisable,
            Npc, NpcInteract,
        },
        player::PlayerCharacter,
        DoMoveResult, MovementType,
    },
    events::{InputEvent, Sender},
    map::{object::ObjectId, MovementId, WarpDestination, WorldMap},
    positions::{BoundingBox, Coordinate, Direction, Location, Position},
    script::{ScriptFlag, WorldInstruction, WorldScriptData},
};

use super::{
    battle::BattleEntry,
    chunk::Connection,
    movement::MapMovementResult,
    wild::{WildChances, WildEntry, WildType},
};

use self::{random::WorldRandoms, tile::PaletteDataMap};

pub mod tile;

mod random;

pub type Maps = HashMap<Location, WorldMap>;

pub struct WorldMapManager<R: Rng + SeedableRng + Clone> {
    pub data: WorldMapData,
    pub scripts: WorldScriptData,
    sender: Sender<WorldActions>,
    randoms: WorldRandoms<R>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldMapData {
    pub maps: Maps,
    pub palettes: PaletteDataMap,
    pub npc: WorldNpcData,
    pub wild: WildChances,
    pub spawn: (Location, Position),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldNpcData {
    pub groups: HashMap<NpcGroupId, NpcGroup>,
    pub trainers: HashMap<TrainerGroupId, TrainerGroup>,
}

impl<R: Rng + SeedableRng + Clone> WorldMapManager<R> {
    pub fn new(data: WorldMapData, scripts: WorldScriptData, sender: Sender<WorldActions>) -> Self {
        Self {
            data,
            scripts,
            sender,
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

    pub fn on_warp(&mut self, player: &mut PlayerCharacter) {
        self.on_map_change(player);
        self.on_tile(player);
    }

    pub fn on_map_change(&self, player: &PlayerCharacter) {
        if let Some(map) = self.data.maps.get(&player.location) {
            self.on_change(map);
        }
    }

    pub fn on_change(&self, map: &WorldMap) {
        self.sender.send(WorldActions::PlayMusic(map.music));
    }

    pub fn input(&mut self, player: &mut PlayerCharacter, input: InputEvent) {
        match input {
            InputEvent::Move(direction) => self.try_move_player(player, direction),
            InputEvent::Interact => player.character.queue_interact(true),
        }
    }

    pub fn try_interact(&mut self, player: &mut PlayerCharacter) {
        // if !player.locks.contains(&Character::INTERACT_LOCK) {
        if let Some(map) = self.data.maps.get_mut(&player.location) {
            let pos = if map
                .tile(player.position.coords)
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
                player.position.in_direction(player.position.direction)
            } else {
                player.position
            };
            for npc in map.npcs.values_mut() {
                if npc.character.interact_from(&pos) {
                    break;
                }
            }
            let forward = player
                .position
                .coords
                .in_direction(player.position.direction);

            if let Some(object) = map.object_at(&forward) {
                const TREE: &ObjectId = unsafe { &ObjectId::new_unchecked(1701147252) };
                const CUT: &MoveId = unsafe { &MoveId::new_unchecked(7632227) };

                const ROCK: &ObjectId = unsafe { &ObjectId::new_unchecked(1801678706) };
                /// "rock-smash"
                const ROCK_SMASH: &MoveId =
                    unsafe { &MoveId::new_unchecked(493254510180952753532786) };

                fn try_break(
                    sender: &Sender<WorldActions>,
                    location: &Location,
                    coordinate: Coordinate,
                    id: &MoveId,
                    player: &mut PlayerCharacter,
                ) {
                    if player
                        .trainer
                        .party
                        .iter()
                        .any(|p| p.moves.iter().any(|m| &m.0 == id))
                    {
                        sender.send(WorldActions::BreakObject(coordinate));
                        player.world.insert_object(location, coordinate);
                    }
                }

                match &object.group {
                    TREE => try_break(&self.sender, &map.id, forward, CUT, player),
                    ROCK => try_break(&self.sender, &map.id, forward, ROCK_SMASH, player),
                    _ => (),
                }
            }

            if let Some(item) = map.item_at(&forward) {
                if !player.world.contains_object(&map.id, &forward) {
                    player.world.insert_object(&map.id, forward);
                    let bag = &mut player.trainer.bag;
                    bag.insert_saved(item.item);
                }
            }
        }
    }

    fn npc_interact(
        scripts: &WorldScriptData,
        player: &mut PlayerCharacter,
        npc: &mut Npc,
        result: Option<DoMoveResult>,
    ) {
        match result {
            Some(result) => match result {
                DoMoveResult::Finished => (),
                DoMoveResult::Interact => match &npc.interact {
                    NpcInteract::Script(script) => {
                        if !player.world.scripts.environment.running() {
                            if let Some(instructions) = scripts.scripts.get(script) {
                                npc.character.on_interact();
                                let env = &mut player.world.scripts.environment;
                                env.executor = Some(npc.id);
                                env.queue = instructions.clone();
                            } else {
                                log::warn!(
                                    "Could not get script with id {} for NPC {}",
                                    script,
                                    npc.character.name
                                );
                            }
                        }
                    }
                    NpcInteract::Nothing => (),
                },
            },
            None => (),
        }
    }

    pub fn move_npcs(&mut self, player: &mut PlayerCharacter, delta: f32) {
        if let Some(map) = self.data.maps.get_mut(&player.location) {
            // Move Npcs

            for npc in player
                .world
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

            match player.world.npc.timer > 0.0 {
                false => {
                    player.world.npc.timer += 1.0;

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

                                            if bb.contains(&next) && next != player.position.coords {
                                                if let Some(code) = map.movements.get(
                                                    npc.character.position.coords.x as usize
                                                        + npc.character.position.coords.y as usize
                                                            * map.width as usize,
                                                ) {
                                                    if WorldMap::can_move(
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
                true => player.world.npc.timer -= delta,
            }
        }
    }

    pub fn update_script(&mut self, player: &mut PlayerCharacter) {
        let env = &mut player.world.scripts.environment;
        let variables = &mut player.world.scripts.flags;
        let queue = &mut env.queue;

        fn insert(queue: &mut Vec<WorldInstruction>, script: &Vec<WorldInstruction>) {
            let cont = script.contains(&WorldInstruction::End);
            match cont {
                true => {
                    *queue = script.clone();
                }
                false => {
                    let mut new = Vec::with_capacity(queue.len() + script.len());
                    new.extend(script.iter().cloned());
                    new.append(queue);
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
                    variables.insert(id.clone(), ScriptFlag::Var(*var));
                    queue.remove(0);
                }
                WorldInstruction::SpecialVar(name, func) => {
                    match func.as_str() {
                        "ShouldTryRematchBattle" => {
                            // to - do: rematches
                            variables.insert(
                                name.clone(),
                                ScriptFlag::Var(match variables.contains_key("REMATCHES") {
                                    true => 1,
                                    false => 0,
                                }),
                            );
                        }
                        _ => (),
                    }
                    queue.remove(0);
                }
                WorldInstruction::Compare(name, var) => {
                    if variables.get(name) == Some(&ScriptFlag::Var(*var)) {
                        variables.insert("EQ".to_owned(), ScriptFlag::Flag);
                    }
                    queue.remove(0);
                }
                WorldInstruction::GotoIfEq(script) => {
                    if variables.remove("EQ").is_some() {
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
                    } else {
                        queue.remove(0);
                    }
                }
                WorldInstruction::Return => {
                    if queue.len() == 1 {
                        queue.push(WorldInstruction::End);
                    }
                    queue.remove(0);
                }
                /*WorldInstruction::Walk(..) | WorldInstruction::FacePlayer | WorldInstruction::TrainerBattleSingle | WorldInstruction::Msgbox(..) */
                npc_inst => {
                    if let Some((map, npc)) = env
                        .executor
                        .as_ref()
                        .map(|id| {
                            self.data
                                .maps
                                .get_mut(&player.location)
                                .map(|map| map.npcs.get_mut(id).map(|npc| (&map.id, npc)))
                                .flatten()
                        })
                        .flatten()
                    {
                        match npc_inst {
                            WorldInstruction::TrainerBattleSingle => {
                                match variables.contains_key("TEMP_MESSAGE") {
                                    true => {
                                        if player.world.message.is_none() {
                                            variables.remove("TEMP_MESSAGE");
                                            if let Some(entry) = BattleEntry::trainer(
                                                &mut player.world.battle,
                                                map,
                                                &npc.id,
                                                npc,
                                            ) {
                                                self.sender.send(WorldActions::Battle(entry));
                                            }
                                        //     let this = &mut queue[0];
                                        //     *this = WorldInstruction::End;
                                        //     self.update_script(player);
                                            queue.remove(0);
                                        }
                                    }
                                    false => {
                                        if player.trainer.party.is_empty()
                                            || player.world.battle.battled(map, &npc.id)
                                        {
                                            queue.remove(0);
                                        } else if let Some(trainer) = npc.trainer.as_ref() {
                                            drop(variables);
                                            let message = MessageState::new(
                                                1,
                                                trainer
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
                                                            .map(|g| g.message)
                                                            .unwrap_or(MessageColor::Black),
                                                    })
                                                    .collect::<Vec<_>>(),
                                            );
                                            player.world.message = Some(message);
                                            player.world.scripts.flags.insert(
                                                "TEMP_MESSAGE".to_owned(),
                                                ScriptFlag::Flag,
                                            );
                                        }
                                    }
                                }
                            }
                            WorldInstruction::Msgbox(m_id, msgbox_type) => {
                                
                                if player.world.message.is_none() {
                                    match variables.contains_key("TEMP_MESSAGE") {
                                        true => {
                                            variables.remove("TEMP_MESSAGE");
                                            queue.remove(0);
                                        }
                                        false => {
                                            match self.scripts.messages.get(m_id) {
                                                Some(message) => {
                                                    let message = MessageState::new(
                                                        1,
                                                        message
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
                                                                    .map(|g| g.message)
                                                                    .unwrap_or(MessageColor::Black),
                                                            })
                                                            .collect::<Vec<_>>(),
                                                    );
                                                    player.world.message = Some(message);
                                                    player.world.scripts.flags.insert(
                                                        "TEMP_MESSAGE".to_owned(),
                                                        ScriptFlag::Flag,
                                                    );
                                                }
                                                None => {
                                                    queue.remove(0);
                                                }
                                            }
                                        }
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
                                    match variables.contains_key("TEMP_0") {
                                        true => {
                                            variables.remove("TEMP_0");
                                            queue.remove(0);
                                        }
                                        false => {
                                            npc.character
                                                .actions
                                                .queue
                                                .push(ActionQueue::Move(*direction));
                                            variables.insert("TEMP_0".to_owned(), ScriptFlag::Flag);
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

    pub fn on_tile(
        &mut self,
        player: &mut PlayerCharacter,
        // party: &[OwnedPokemon<P, M, I>],
    ) {
        self.sender.send(WorldActions::OnTile);

        if let Some(map) = self.data.maps.get_mut(&player.location) {
            if player.world.wild.encounters {
                if let Some(current) = map.tile(player.position.coords) {
                    if self
                        .data
                        .palettes
                        .get(current.palette(&map.palettes))
                        .map(|data| data.wild.contains(&current.id()))
                        .unwrap_or_default()
                    {
                        let t = match player.movement {
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
                                self.sender.send(WorldActions::Battle(entry));
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

    fn stop_player(&mut self, player: &mut PlayerCharacter) {
        player.stop_move();

        if let Some(map) = self.data.maps.get(&player.location) {
            if let Some(destination) = map.warp_at(&player.position.coords) {
                // Warping does not trigger tile actions!
                player.world.warp = Some(*destination);
            } else if map.in_bounds(player.position.coords) {
                self.on_tile(player);
            }
        }
    }

    pub fn update(&mut self, player: &mut PlayerCharacter, delta: f32) {
        if let Some(result) = player.do_move(delta) {
            match result {
                DoMoveResult::Finished => self.stop_player(player),
                DoMoveResult::Interact => self.try_interact(player),
            }
        }
        self.move_npcs(player, delta);
        self.update_script(player);
        // self.update_interactions(player);
    }

    pub fn try_move_player(&mut self, player: &mut PlayerCharacter, direction: Direction) {
        player.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = player.position.coords + offset;

        if let Some(map) = self.get(&player.location) {
            // Check for warp on tile
            if player.world.warp.is_none() {
                if let Some(destination) = map.warp_at(&coords) {
                    player.world.warp = Some(*destination);
                    self.sender.send(WorldActions::BeginWarpTransition(coords));
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
                && !player.noclip
            {
                self.sender.send(WorldActions::PlayerJump);
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
                            self.on_map_change(&player);
                        }
                    }
                }
            }
        }

        fn with_code(player: &mut PlayerCharacter, code: MovementId, direction: Direction) -> bool {
            if WorldMap::can_move(player.position.elevation, code) || player.noclip {
                if WorldMap::WATER == code {
                    if player.movement != MovementType::Swimming {
                        const SURF: &MoveId = unsafe { &MoveId::new_unchecked(1718777203) };

                        if player
                            .trainer
                            .party
                            .iter()
                            .flat_map(|pokemon| pokemon.moves.iter())
                            .any(|m| &m.0 == SURF)
                            || player.noclip
                        {
                            player.movement = MovementType::Swimming;
                        } else {
                            return false;
                        }
                    }
                } else if player.movement == MovementType::Swimming {
                    player.movement = MovementType::Walking;
                }
                WorldMap::change_elevation(&mut player.position.elevation, code);
                player.actions.queue.push(ActionQueue::Move(direction));
                return true;
                // self.player.offset =
                //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
            }
            false
        }
    }

    pub fn warp(&mut self, player: &mut PlayerCharacter, destination: WarpDestination) {
        if self.data.warp(player, destination) {
            self.on_warp(player);
        }
    }
}

impl WorldMapData {
    pub fn connection_movement(
        &self,
        direction: Direction,
        offset: i32,
        connections: &[Connection],
        player: &PlayerCharacter,
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

    pub fn post_battle(&mut self, player: &mut PlayerCharacter, winner: bool) {
        player.character.locked.decrement();
        let entry = player.world.battle.battling.take();
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
                                    player.world.battle.insert(&trainer.location, trainer.id);
                                }
                                TrainerDisable::Many(others) => {
                                    player.world.battle.insert(&trainer.location, trainer.id);
                                    player
                                        .world
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
            let (loc, pos) = player.world.heal.unwrap_or(self.spawn);
            player.location = loc;
            player.position = pos;
            player.location = player.location;
            player
                .trainer
                .party
                .iter_mut()
                .for_each(|o| o.heal(None, None));
        }
    }

    pub fn warp(&self, player: &mut PlayerCharacter, destination: WarpDestination) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                player.warp(destination);
                true
            }
            false => false,
        }
    }
}
