use firecore_pokedex::trainer::InitTrainer;
use firecore_text::{MessagePage, MessageState, MessageStates};
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        action::ActionQueue,
        message::process_str_player,
        npc::{Npc, NpcId},
        DoMoveResult,
    },
    map::{battle::BattleEntry, data::WorldMapData},
    message::MessageTheme,
    positions::{Coordinate, Location},
    random::WorldRandoms,
    state::map::{MapEvent, MapState},
};

use super::WorldScriptingEngine;

pub use self::condition::Condition;
pub use self::instructions::*;

mod condition;
mod instructions;

pub type ScriptId = String;
pub type MessageId = String;

pub type VariableName = String;
pub type Variable = u16;

pub type Flag = String;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultScriptState {
    pub executed: HashSet<ScriptId>,
    pub npcs: HashMap<NpcId, (Location, Npc)>,
    pub flags: HashSet<VariableName>,
    pub variables: HashMap<VariableName, Variable>,
    #[deprecated]
    pub executor: Option<NpcId>,
    pub queue: Vec<WorldInstruction>,
}
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub enum ScriptVariable {
//     /// Flag that shows that a variable exists
//     Flag,
//     Var(u16),
// }

impl DefaultScriptState {
    pub fn stop(&mut self) {
        self.executor = None;
        self.queue.clear();
    }

    pub fn running(&self) -> bool {
        self.executor.is_some() || !self.queue.is_empty()
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WorldScript {
//     pub identifier: ScriptId,
//     pub actions: Vec<WorldInstruction>,
// }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultWorldScriptEngine {
    pub locations: HashMap<Location, ScriptLocation>,
    pub scripts: HashMap<ScriptId, Vec<WorldInstruction>>,
    pub messages: HashMap<MessageId, Vec<Vec<String>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScriptLocation {
    pub tiles: HashMap<Coordinate, ScriptId>,
    pub npcs: HashMap<NpcId, ScriptId>,
}

impl WorldScriptingEngine for DefaultWorldScriptEngine {
    type State = DefaultScriptState;

    type Error = ();

    fn on_tile(
        &self,
        world: &mut MapState,
        state: &mut Self::State,
    ) {
        if let Some(scriptid) = self
            .locations
            .get(&world.location)
            .and_then(|location| location.tiles.get(&world.player.character.position.coords))
        {
            self.run(world, state, scriptid, None);
        }
    }

    fn update<R: rand::Rng>(
        &self,
        data: &WorldMapData,
        world: &mut MapState,
        trainer: &mut InitTrainer,
        randoms: &mut WorldRandoms<R>,
        state: &mut Self::State,
    ) {
        if !world.npc.results.is_empty() {
            let (id, result) = world.npc.results.remove(0);
            match result {
                DoMoveResult::Finished => (),
                DoMoveResult::Interact => {
                    if let Some(scriptid) = self
                        .locations
                        .get(&world.location)
                        .and_then(|location| location.npcs.get(&id))
                    {
                        self.run(world, state, scriptid, Some(id));
                    }
                }
            }
        }

        let variables = &mut state.variables;
        let queue = &mut state.queue;

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
                    if let Some(ref executor) = state.executor.take() {
                        if let Some(character) = world
                            .entities
                            .get_mut(&world.location)
                            .and_then(|state| state.npcs.get_mut(executor))
                        {
                            character.end_interact()
                        }
                    }
                    queue.remove(0);
                }
                WorldInstruction::Lock => {
                    world.player.character.locked.increment();
                    queue.remove(0);
                }
                WorldInstruction::Release => {
                    world.player.character.locked.decrement();
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
                                match state.flags.contains("REMATCHES") {
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
                        state.flags.insert("TEMP_EQ".to_owned());
                    }
                    queue.remove(0);
                }
                WorldInstruction::GotoIfEq(script) => {
                    if state.flags.remove("TEMP_EQ") {
                        match self.scripts.get(script) {
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
                        match self.scripts.get(script) {
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
                    state.flags.insert(flag.clone());
                    queue.remove(0);
                }
                WorldInstruction::Call(script) => {
                    match self.scripts.get(script) {
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
                    world.events.push(MapEvent::PlaySound(*id, *variant));
                    queue.remove(0);
                }
                WorldInstruction::WaitFanfare() => {
                    log::warn!("Add WaitFanfare instruction!");
                    queue.remove(0);
                }
                WorldInstruction::AddItem(item) => {
                    // world
                    //     .events
                    //     .push(MapEvent::GiveItem(SavedItemStack::from(*item)));
                    log::debug!("additem instruction");
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
                // WorldInstruction::Walk(..) | WorldInstruction::FacePlayer | WorldInstruction::TrainerBattleSingle | WorldInstruction::Msgbox(..) | WorldInstruction::Look(..) => {
                npc_inst => {
                    if let Some((map, settings, npc)) = state
                        .executor
                        .as_ref()
                        .map(|id| {
                            data.maps
                                .get(&world.location)
                                .map(|map| {
                                    map.npcs.get(id).map(|npc| (&map.id, &map.settings, npc))
                                })
                                .flatten()
                        })
                        .flatten()
                    {
                        match npc_inst {
                            
                            WorldInstruction::ApplyMovement(id, movement) => {
                                log::debug!("applymovement");
                                queue.remove(0);
                            },
                            WorldInstruction::WaitMovement(id) => {
                                match world.entities.get_mut(&world.location).and_then(|states| {
                                    states.npcs.get_mut(&id)
                                }) {
                                    Some(npc) => {
                                        if !npc.moving() {
                                            queue.remove(0);
                                        }
                                    },
                                    None => {
                                        queue.remove(0);
                                    },
                                }
                            }
                            WorldInstruction::LockAll => {
                                world.player.character.input_lock.increment();
                                if let Some(entities) = world.entities.get_mut(&world.location) {
                                    for character in entities.npcs.values_mut() {
                                        character.input_lock.increment();
                                    }
                                }
                                queue.remove(0);
                            },
                            WorldInstruction::ReleaseAll => {
                                world.player.character.input_lock.decrement();
                                if let Some(entities) = world.entities.get_mut(&world.location) {
                                    for character in entities.npcs.values_mut() {
                                        character.input_lock.decrement();
                                    }
                                }
                                queue.remove(0);
                            },
                            WorldInstruction::TrainerBattleSingle => {
                                match state.flags.contains("TEMP_MESSAGE") {
                                    true => {
                                        if !world.message.is_running() {
                                            state.flags.remove("TEMP_MESSAGE");
                                            if let Some(entry) = BattleEntry::trainer(
                                                &mut world.player.battle,
                                                map,
                                                &settings,
                                                &data.npc,
                                                &npc.id,
                                                npc,
                                            ) {
                                                world.player.battle.battling = Some(entry);
                                            }
                                            //     let this = &mut queue[0];
                                            //     *this = WorldInstruction::End;
                                            //     self.update_script(player);
                                            queue.remove(0);
                                        }
                                    }
                                    false => {
                                        if trainer.party.is_empty()
                                            || world.player.battle.battled(map, &npc.id)
                                        {
                                            queue.remove(0);
                                        } else if let Some(trainer) = npc.trainer.as_ref() {
                                            drop(variables);
                                            state.flags.insert("TEMP_MESSAGE".to_owned());
                                            let message = MessageState {
                                                pages: trainer
                                                    .encounter
                                                    .iter()
                                                    .map(|lines| MessagePage {
                                                        lines: lines
                                                            .iter()
                                                            .map(|str| {
                                                                process_str_player(
                                                                    str,
                                                                    &world.player,
                                                                )
                                                            })
                                                            .collect(),
                                                        wait: None,
                                                        color: data
                                                            .npc
                                                            .groups
                                                            .get(&npc.group)
                                                            .map(|g| g.message),
                                                        theme: MessageTheme::default(),
                                                    })
                                                    .collect::<Vec<_>>(),
                                                ..Default::default()
                                            };
                                            world.message = MessageStates::Running(message);
                                        }
                                    }
                                }
                            }
                            WorldInstruction::Msgbox(m_id, msgbox_type) => {
                                if !world.message.is_running() {
                                    match state.flags.contains("TEMP_MESSAGE") {
                                        true => {
                                            state.flags.remove("TEMP_MESSAGE");
                                            queue.remove(0);
                                        }
                                        false => match self.messages.get(m_id) {
                                            Some(message) => {
                                                let theme =
                                                    MessageTheme::new(msgbox_type.as_deref())
                                                        .unwrap_or_default();
                                                let message = MessageState {
                                                    pages: message
                                                        .iter()
                                                        .map(|lines| MessagePage {
                                                            lines: lines
                                                                .iter()
                                                                .map(|str| {
                                                                    process_str_player(
                                                                        str,
                                                                        &world.player,
                                                                    )
                                                                })
                                                                .collect(),
                                                            wait: None,
                                                            color: data
                                                                .npc
                                                                .groups
                                                                .get(&npc.group)
                                                                .map(|g| g.message),
                                                            theme,
                                                        })
                                                        .collect::<Vec<_>>(),
                                                    ..Default::default()
                                                };
                                                world.message = MessageStates::Running(message);
                                                state.flags.insert("TEMP_MESSAGE".to_owned());
                                            }
                                            None => {
                                                queue.remove(0);
                                            }
                                        },
                                    }
                                }
                            }
                            WorldInstruction::FacePlayer => {
                                // let pos = &mut character.position;
                                // pos.direction =
                                //     pos.coords.towards(player.character.position.coords);
                                queue.remove(0);
                            }
                            WorldInstruction::Look(direction) => {
                                if let Some(character) =
                                    world.entities.get_mut(&world.location).and_then(|states| {
                                        states.npcs.get_mut(state.executor.as_ref().unwrap())
                                    })
                                {
                                    if !character.moving() {
                                        character.position.direction = *direction;
                                    }
                                }
                            }
                            WorldInstruction::Walk(direction) => {
                                if let Some(character) =
                                    world.entities.get_mut(&world.location).and_then(|states| {
                                        states.npcs.get_mut(state.executor.as_ref().unwrap())
                                    })
                                {
                                    if !character.moving() {
                                        match state.flags.contains("TEMP_0") {
                                            true => {
                                                state.flags.remove("TEMP_0");
                                                queue.remove(0);
                                            }
                                            false => {
                                                character
                                                    .actions
                                                    .queue
                                                    .push(ActionQueue::Move(*direction));
                                                state.flags.insert("TEMP_0".to_owned());
                                            }
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        } else if state.executor.is_some() {
            state.executor = None;
        }
    }
}

impl DefaultWorldScriptEngine {

    fn run(&self, world: &mut MapState, state: &mut <Self as WorldScriptingEngine>::State, scriptid: &str, executor: Option<NpcId>) {
        match state.running() {
            false => match self.scripts.get(scriptid) {
                Some(instructions) => {
                    if let Some(character) = executor.as_ref().and_then(|id| world
                        .entities
                        .get_mut(&world.location)
                        .and_then(|state| state.npcs.get_mut(&id)))
                    {
                        character.on_interact();
                    }
                    state.executor = executor;
                    state.queue = instructions.clone();
                }
                None => {
                    log::warn!(
                        "Could not get script with id {} for executor id {:?}",
                        scriptid,
                        executor
                    );
                    world.player.character.input_lock.decrement();
                }
            },
            true => {
                log::debug!("Could not run script as one is running already!")
            }
        }
    }

}