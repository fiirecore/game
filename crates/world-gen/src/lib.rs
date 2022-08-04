use std::{num::ParseIntError, sync::Arc};

use dashmap::DashMap;
use hashbrown::{hash_map::DefaultHashBuilder as RandomState, HashMap};

use bin::BinaryMap;

use firecore_world::{
    character::npc::{
        group::TrainerGroupId,
        trainer::{NpcTrainer, TrainerDisable},
        Npc, NpcMovement, Npcs,
    },
    map::{
        chunk::{ChunkConnections, Connection, WorldChunk},
        movement::Elevation,
        object::*,
        warp::{WarpDestination, WarpEntry},
        wild::{WildEntry, WildType},
        Brightness, PaletteId, WorldMap, WorldMapSettings, WorldTile,
    },
    pokedex::{
        item::Item,
        moves::{owned::SavedMove, Move},
        pokemon::{owned::SavedPokemon, stat::StatSet, Pokemon},
        trainer::Trainer,
        Dex,
    },
    positions::{BoundingBox, Coordinate, Destination, Direction, Location, Position},
    script::default::*,
};
use map::{
    object::JsonObjectEvent, warp::JsonWarpEvent, wild::JsonWildEncounters, JsonConnection, JsonMap,
};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde_json::Value;
use tinystr::TinyStr16;

use crate::{
    map::JsonMapLayout,
    script::inc::{Command, Script},
};

const PATH: &str = "http://raw.githubusercontent.com/pret/pokefirered/master";

mod edits;
mod map;
mod mapping;

mod script;

mod bin;
mod structs;

pub use edits::*;
pub use mapping::*;
// mod serializable;

type Maps = DashMap<String, JsonMap, RandomState>;
type Scripts = DashMap<String, Script, RandomState>;
type Messages = DashMap<String, Vec<Vec<String>>, RandomState>;
type Trainers = HashMap<String, script::trainer::Trainer>;
type Parties = HashMap<String, Vec<script::trainer::party::TrainerPokemon>>;
type NpcScripts = DashMap<Location, HashMap<ObjectId, String>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParsedData {
    pub maps: Maps,
    pub wild: JsonWildEncounters,
    pub pokedex: Dex<Pokemon>,
    pub movedex: Dex<Move>,
    pub itemdex: Dex<Item>,
    pub scripts: Scripts,
    pub messages: Messages,
    pub trainers: Trainers,
    pub parties: Parties,
}

pub struct WorldData {
    pub maps: HashMap<Location, WorldMap>,
    pub scripts: DefaultWorldScriptEngine,
}

pub fn compile(
    mappings: NameMappings,
    edits: edits::Edits,
    mut data: ParsedData,
) -> anyhow::Result<WorldData> {
    println!("Converting wild encounters...");

    eprintln!("TODO: fix fishing encounters");

    let encounters = DashMap::new();

    let wild = std::mem::take(&mut data.wild.wild_encounter_groups);

    wild.into_par_iter()
        .flat_map(|g| g.encounters.into_par_iter())
        .filter(|e| e.base_label[(e.base_label.len() - 7)..].eq_ignore_ascii_case("FireRed"))
        .for_each(|e| {
            let mut entries = HashMap::new();
            if let Some(e) = e.land_mons {
                entries.insert(WildType::Land, e.into(&data.pokedex));
            }
            if let Some(e) = e.water_mons {
                entries.insert(WildType::Water, e.into(&data.pokedex));
            }
            if let Some(e) = e.rock_smash_mons {
                entries.insert(WildType::Rock, e.into(&data.pokedex));
            }
            if let Some(e) = e.fishing_mons {
                entries.insert(WildType::Fishing(0), e.into(&data.pokedex));
            }
            if entries.is_empty() {
                encounters.insert(e.map, None);
            } else {
                encounters.insert(e.map, Some(entries));
            }
        });

    println!("Created {} wild encounters", encounters.len());

    let new_maps = DashMap::<Location, WorldMap>::new();

    println!("Converting maps...");

    let npc_scripts = DashMap::<Location, _>::new();

    data.maps.par_iter().for_each(|map| {
        let map = map.value();
        println!("Converting {}", map.data.name);
        if let Some((map, scripts)) = into_world_map(&mappings, &data, &encounters, map) {
            let id = map.id;
            if let Some(removed) = new_maps.insert(id, map) {
                panic!("Duplicate world map id {}", removed.id);
            }
            if let Some(removed) = npc_scripts.insert(id, scripts) {
                panic!("Duplicate world map id for scripts: {}", id);
            }
        } else {
            eprintln!("Could not convert {} into a world map", map.data.name);
        }
    });

    println!("Editing maps...");

    edits.process(&new_maps);

    // println!("Saving maps...");

    // serializable::serialize(
    //     "output",
    //     &new_maps,
    //     &mappings,
    //     &data.scripts,
    //     &data.messages,
    // );

    println!("Done!");

    Ok(WorldData {
        maps: new_maps.into_par_iter().collect(),
        scripts: create_world_script_data(&mappings, &data.scripts, &data.messages, &npc_scripts),
    })
}

#[derive(Debug, Clone, Copy)]
enum OptionError {
    Unknown,
}

impl std::error::Error for OptionError {}

impl core::fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

pub fn create_data() -> anyhow::Result<ParsedData> {
    eprintln!("Parsed map file cannot be read!");
    eprintln!("Generating new parsed map file...");

    println!("Loading dex...");

    let client = firecore_dex_gen::client();

    let pokemon = firecore_dex_gen::pokemon::generate(client.clone(), 1..386);
    let moves = firecore_dex_gen::moves::generate(client, 1..559);
    let items = firecore_dex_gen::items::generate();

    let pokedex = Dex(pokemon.into_iter().map(|p| (p.id, Arc::new(p))).collect());
    let movedex = Dex(moves.into_iter().map(|m| (m.id, Arc::new(m))).collect());
    let itemdex = Dex(items.into_iter().map(|i| (i.id, Arc::new(i))).collect());

    // let (pokedex, movedex, itemdex) = firecore_storage::from_bytes::<(
    //     Dex<Pokemon, _>,
    //     Dex<Move, _>,
    //     Dex<Item, _>,
    // )>(&dex)
    // .unwrap();

    println!("Getting trainers...");

    let trainers = attohttpc::get(format!("{}/src/data/trainers.h", PATH))
        .send()?
        .text_utf8()?;
    let trainers = script::trainer::parse_trainers(&trainers)?;

    println!("Getting trainer parties...");

    let parties = attohttpc::get(format!("{}/src/data/trainer_parties.h", PATH))
        .send()?
        .text_utf8()?;
    let parties = script::trainer::party::parse_parties(&parties)?;

    println!("Getting layouts...");

    let layouts = attohttpc::get(format!("{}/data/layouts/layouts.json", PATH))
        .send()?
        .json::<map::JsonMapLayouts>()?;

    println!("Getting map groups...");

    let maps = attohttpc::get(format!("{}/data/maps/map_groups.json", PATH))
        .send()?
        .bytes()?;

    println!("Getting wild encounters...");

    let wild = attohttpc::get(format!("{}/src/data/wild_encounters.json", PATH))
        .send()?
        .json::<JsonWildEncounters>()?;

    println!("Parsing map groups...");

    let maps = serde_json::from_slice::<Value>(&maps)?;

    let mut names = Vec::new();

    for group_name in maps
        .get("group_order")
        .ok_or(OptionError::Unknown)?
        .as_array()
        .ok_or(OptionError::Unknown)?
    {
        for name in maps
            .get(group_name.as_str().ok_or(OptionError::Unknown)?)
            .ok_or(OptionError::Unknown)?
            .as_array()
            .ok_or(OptionError::Unknown)?
        {
            names.push(name.as_str().ok_or(OptionError::Unknown)?);
        }
    }

    println!("Found {} map names", names.len());

    let maps: Maps = Default::default();
    let mut scripts: Scripts = Default::default();
    let messages: Messages = Default::default();

    let layouts = layouts
        .layouts
        .into_par_iter()
        .flat_map(|l| l.inner.left())
        .map(|l| (l.id.clone(), l))
        .collect::<DashMap<String, JsonMapLayout, RandomState>>();

    names.into_par_iter().for_each(|map| {
        let path = format!("{}/data/maps/{}/map.json", PATH, map);
        let scripts_path = format!("{}/data/maps/{}/scripts.inc", PATH, map);
        let text_path = format!("{}/data/maps/{}/text.inc", PATH, map);

        let data = attohttpc::get(path)
            .send()
            .unwrap_or_else(|err| panic!("Could not get {} with error {}", map, err))
            .json::<map::JsonMapData>()
            .unwrap_or_else(|err| panic!("Could not get {} with error {}", map, err));

        if let Some(scripts_data) = attohttpc::get(scripts_path)
            .send()
            .ok()
            .map(|r| r.text().ok())
            .flatten()
        {
            match script::inc::parse(&scripts_data) {
                Ok(scripts_data) => {
                    for script in scripts_data {
                        scripts.insert(script.name.clone(), script);
                    }
                }
                Err(err) => {
                    println!("Could not parse script for map {} with error {}", map, err)
                }
            }
        }

        if let Some(message_data) = attohttpc::get(text_path)
            .send()
            .ok()
            .map(|r| r.text().ok())
            .flatten()
        {
            if let Ok(message_data) = script::inc::parse_message_script(&message_data) {
                for message in message_data {
                    messages.insert(message.name, message.text);
                }
            }
        }

        let layout = layouts
            .get(&data.layout)
            .unwrap_or_else(|| panic!("Could not get map layout {}", data.layout));

        let layout = layout.value().clone();

        println!("Parsed map {}", data.name);

        if let Some(removed) = maps.insert(data.id.clone(), JsonMap { data, layout }) {
            panic!("Map {} was removed!", removed.data.name);
        }
    });

    println!("Getting trainer scripts...");

    let trainer_scripts = attohttpc::get(format!("{}/data/scripts/trainers.inc", PATH))
        .send()?
        .text_utf8()?;

    println!("Parsing trainer scripts...");

    scripts.extend(
        script::inc::parse(&trainer_scripts)?
            .into_iter()
            .map(|s| (s.name.clone(), s)),
    );

    let data = ParsedData {
        maps,
        wild,
        pokedex,
        movedex,
        itemdex,
        scripts,
        messages,
        trainers,
        parties,
    };

    println!("Done parsing maps!");

    // std::fs::write(PARSED, firecore_storage::to_bytes(&data).unwrap()).unwrap();

    // std::fs::write(
    //     "output/parsed.ron",
    //     ron::ser::to_string_pretty(&data, Default::default()).unwrap(),
    // )
    // .unwrap();

    Ok(data)
}

fn into_world_map(
    mappings: &NameMappings,
    data: &ParsedData,
    encounters: &DashMap<String, Option<HashMap<WildType, WildEntry>>>,
    map: &JsonMap,
) -> Option<(WorldMap, HashMap<ObjectId, String>)> {
    let map_path = format!("{}/{}", PATH, map.layout.blockdata_filepath);
    let border_path = format!("{}/{}", PATH, map.layout.border_filepath);

    let map_data = attohttpc::get(map_path).send().unwrap().bytes().unwrap();
    let border_data = attohttpc::get(border_path).send().unwrap().bytes().unwrap();

    let mapdata = BinaryMap::load(
        &map_data,
        &border_data,
        map.layout.width * map.layout.height,
    )?;

    let palettes = into_palettes(
        mappings,
        &map.layout.primary_tileset,
        &map.layout.secondary_tileset,
    );

    let id = mappings
        .map
        .id
        .get(&map.data.id)
        .cloned()
        .unwrap_or_else(|| loc(&map.data.id));

    let border = mapdata
        .border
        .tiles
        .into_iter()
        .map(|tile| {
            let size = *mappings.palettes.sizes.get(&palettes[0]).unwrap();
            match size > tile {
                false => WorldTile::Secondary(tile - size),
                true => WorldTile::Primary(tile),
            }
        })
        .collect::<Vec<_>>();

    let (npcs, scripts) = into_world_npcs(mappings, data, &map.data.object_events);

    Some((
        WorldMap {
            id,
            name: mappings
                .map
                .name
                .get(&map.data.name)
                .unwrap_or(&map.data.name)
                // .unwrap_or_else(|| panic!("Cannot get map name mapping for {}", map.data.name))
                .clone(),
            music: into_music(mappings, &map.data.music),
            width: map.layout.width as _,
            height: map.layout.height as _,
            tiles: mapdata
                .tiles
                .into_iter()
                .map(|tile| {
                    let palette = *mappings.palettes.sizes.get(&palettes[0]).unwrap();
                    match palette > tile {
                        false => WorldTile::Secondary(tile - palette),
                        true => WorldTile::Primary(tile),
                    }
                })
                .collect(),
            palettes,
            movements: mapdata.movements,
            border: [border[0], border[1], border[2], border[3]],
            chunk: map
                .data
                .connections
                .as_ref()
                .map(|connections| into_chunk(mappings, connections))
                .flatten(),
            warps: map
                .data
                .warp_events
                .iter()
                .flat_map(|warp| into_world_warp(mappings, &data.maps, warp))
                .collect(),
            wild: encounters.remove(&map.data.id).map(|(.., v)| v).flatten(),
            npcs,
            // objects: into_world_objects(mappings, &map.data.object_events),
            // items: into_world_items(data, &map.data.bg_events),
            // signs: into_world_signs(data, &map.data.bg_events),
            settings: WorldMapSettings {
                fly_position: None,
                brightness: match map.data.weather == "WEATHER_SHADE" {
                    true => Brightness::Night,
                    false => Brightness::Day,
                },
                transition: mappings
                    .map
                    .transition
                    .get(&map.data.battle_scene)
                    .copied()
                    .unwrap_or_else(|| WorldMapSettings::default_transition()),
            },
            // scripts: Default::default(),
        },
        scripts,
    ))
}

fn create_world_script_data(
    mappings: &NameMappings,
    scripts: &Scripts,
    messages: &Messages,
    npc_scripts: &NpcScripts,
) -> DefaultWorldScriptEngine {
    DefaultWorldScriptEngine {
        scripts: scripts
            .par_iter()
            .flat_map(|r| {
                let k = r.key();
                let k = k.clone();
                let v = r.value();
                let args: Vec<_> = v
                    .commands
                    .iter()
                    .map(|c| match into_instruction(mappings, &k, c) {
                        Ok(i) => Some(i),
                        Err(err) => match err {
                            InstructionError::Unknown(..) => None,
                            InstructionError::ParseInt(..) => None,
                            err => panic!("{}", err),
                        },
                    })
                    .collect();
                if args.contains(&None) {
                    return None;
                }
                (!args.is_empty()).then(|| (k, args.into_iter().flatten().collect()))
            })
            .collect(),
        messages: messages
            .par_iter()
            .map(|r| (r.key().clone(), r.value().clone()))
            .collect(),
        npcs: npc_scripts
            .par_iter()
            .map(|r| (r.key().clone(), r.value().clone()))
            .collect(),
    }
}

fn into_instruction(
    mappings: &crate::NameMappings,
    id: &ScriptId,
    command: &Command,
) -> Result<WorldInstruction, InstructionError> {
    Ok(match command.command.as_str() {
        "end" | "step_end" => WorldInstruction::End,
        "return" => WorldInstruction::Return,
        // set variables
        "setvar" => WorldInstruction::SetVar(
            command.arguments[0].clone(),
            command.arguments[1].parse().map_err(|err| {
                InstructionError::ParseInt(id.clone(), command.arguments[1].clone(), err)
            })?,
        ),
        "setflag" => WorldInstruction::SetFlag(command.arguments[0].clone()),
        "specialvar" => {
            WorldInstruction::SpecialVar(command.arguments[0].clone(), command.arguments[1].clone())
        }
        // compare
        "compare" => WorldInstruction::Compare(
            command.arguments[0].clone(),
            match command.arguments[1].as_str() {
                "TRUE" => 1,
                "FALSE" => 0,
                other => other.parse().map_err(|err| {
                    InstructionError::ParseInt(id.clone(), command.arguments[1].clone(), err)
                })?,
            },
        ),
        "call" => WorldInstruction::Call(command.arguments[0].clone()),
        // goto/call
        "goto_if_eq" => WorldInstruction::GotoIfEq(command.arguments[0].clone()),
        "goto_if_set" => {
            WorldInstruction::GotoIfSet(command.arguments[0].clone(), command.arguments[1].clone())
        }
        // Player Freezing
        "lock" => WorldInstruction::Lock,
        "release" => WorldInstruction::Release,
        // NPC commands
        "faceplayer" => WorldInstruction::FacePlayer,
        "walk_down" => WorldInstruction::Walk(Direction::Down),
        "walk_up" => WorldInstruction::Walk(Direction::Up),
        "walk_left" => WorldInstruction::Walk(Direction::Left),
        "walk_right" => WorldInstruction::Walk(Direction::Right),
        "walk_in_place_fastest_up" => WorldInstruction::Walk(Direction::Up),
        // Singular trainer battle
        "trainerbattle_single" => WorldInstruction::TrainerBattleSingle,
        // Message
        "msgbox" => WorldInstruction::Msgbox(
            command.arguments[0].clone(),
            command.arguments.get(1).cloned(),
        ),
        "textcolor" => WorldInstruction::TextColor(command.arguments[0].parse().unwrap()),
        "message" => WorldInstruction::Message(command.arguments[0].clone()),
        "waitmessage" => WorldInstruction::WaitMessage,
        // Sound
        "playfanfare" => {
            let (id, var) = mappings
                .audio
                .sounds
                .get(&command.arguments[0][4..])
                .ok_or(InstructionError::MissingMapping(
                    id.clone(),
                    command.arguments[0][4..].to_owned(),
                ))?;
            WorldInstruction::PlayFanfare(*id, *var)
        }
        "waitfanfare" => WorldInstruction::WaitMessage,
        // Item
        "additem" => WorldInstruction::AddItem(command.arguments[0][5..].parse().unwrap()),
        "checkitemspace" => WorldInstruction::CheckItemSpace(
            command.arguments[0].clone(),
            command.arguments[1].parse().unwrap(),
        ),
        "getitemname" => WorldInstruction::GetItemName(
            command.arguments[0].parse().map_err(|err| {
                InstructionError::ParseInt(id.clone(), command.arguments[0].clone(), err)
            })?,
            command.arguments[1].clone(),
        ),
        com => return Err(InstructionError::Unknown(id.to_string(), com.to_owned())),
    })
}

#[derive(Debug)]
enum InstructionError {
    Unknown(ScriptId, String),
    ParseInt(ScriptId, String, ParseIntError),
    // ParseStr(ScriptId, String, tinystr::TinyStrError),
    MissingMapping(ScriptId, String),
}

impl std::error::Error for InstructionError {}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

fn loc(id: &str) -> Location {
    Location {
        map: Some("unnamed".parse().unwrap()),
        index: truncate_id(id),
    }
}

fn truncate_id(id: &str) -> TinyStr16 {
    let id = &id[4..];
    if id.len() >= 16 {
        format!("{}{}", &id[..12], &id[id.len() - 4..]).parse()
    } else {
        id.parse()
    }
    .unwrap()
}

fn into_chunk(mappings: &NameMappings, json_connections: &[JsonConnection]) -> Option<WorldChunk> {
    match json_connections.is_empty() {
        true => None,
        false => {
            let mut connections = ChunkConnections::new();
            for connection in json_connections {
                let direction = match connection.direction.as_str() {
                    "left" => Direction::Left,
                    "right" => Direction::Right,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => unreachable!(),
                };
                if !connections.contains_key(&direction) {
                    connections.insert(direction, Vec::new());
                }
                connections.get_mut(&direction).unwrap().push(Connection(
                    mappings
                        .map
                        .id
                        .get(&connection.map)
                        .cloned()
                        .unwrap_or_else(|| loc(&connection.map)),
                    connection.offset as _,
                ))
            }
            Some(WorldChunk { connections })
        }
    }
}

fn into_world_warp(
    mappings: &NameMappings,
    maps: &Maps,
    warp: &JsonWarpEvent,
) -> Option<WarpEntry> {
    let destination = mappings
        .map
        .id
        .get(&warp.destination)
        .cloned()
        .unwrap_or_else(|| loc(&warp.destination));

    // let name = format!("warp_{}", index).parse().unwrap();

    let entry = WarpEntry {
        area: BoundingBox {
            min: Coordinate {
                x: warp.x as _,
                y: warp.y as _,
            },
            max: Coordinate {
                x: warp.x as _,
                y: warp.y as _,
            },
        },
        destination: WarpDestination {
            location: destination,
            position: {
                let w = &maps
                    .get(&warp.destination)?
                    // .unwrap_or_else(|| panic!("Cannot get map at {}", warp.destination))
                    .data
                    .warp_events[warp.dest_warp_id as usize];
                Destination {
                    coords: Coordinate {
                        x: w.x as _,
                        y: w.y as _,
                    },
                    direction: None,
                }
            },
            // transition: WarpTransition {
            //     move_on_exit: false,
            //     warp_on_tile: true,
            //     change_music: true,
            // },
        },
    };

    Some(entry)
}

fn into_world_npcs(
    mappings: &NameMappings,
    data: &ParsedData,
    events: &[JsonObjectEvent],
) -> (Npcs, HashMap<ObjectId, String>) {
    let (a, b) = events
        .par_iter()
        .enumerate()
        .flat_map(|(index, event)| {
            if let Some(group) = mappings.npcs.groups.get(&event.graphics_id) {
                let (movement, directions) = mappings
                    .npcs
                    .movement
                    .get(&event.movement_type)
                    .cloned()
                    .unwrap_or_default();

                let mut interact = None;

                let mut trainer = None;
                let mut name = String::new();

                let mut script_name = None;

                if let Some(script) = data.scripts.get(&event.script) {
                    let script = script.value();
                    // if script.commands.len() == 1 {
                    //     let command = &script.commands[0];
                    //     if &command.command == "msgbox" {
                    //         let message = data.messages.get(&command.arguments[0]).unwrap();
                    //         let message = message.value();
                    //         interact = NpcInteract::Message(message.clone());
                    //     }
                    // }

                    if !(event.trainer_type.eq_ignore_ascii_case("TRAINER_TYPE_NONE")) {
                        if let Some(battle) = script.commands.iter().find(|command| {
                            command.command.eq_ignore_ascii_case("trainerbattle_single")
                        }) {
                            let mut args = battle.arguments.iter();
                            let id = args.next().unwrap();
                            let encounter_id = args.next().unwrap();
                            let defeat_id = args.next().unwrap();
                            let t = data.trainers.get(id).unwrap();
                            let party = data
                                .parties
                                .get(&t.party)
                                .unwrap_or_else(|| panic!("Could not get party for {}", id));
                            let sight = event.trainer_sight_or_berry_tree_id.parse().unwrap();
                            if let Some(trainer_name) = &t.name {
                                name = trainer_name.clone();
                            }

                            fn get_group(t: &script::trainer::Trainer) -> TrainerGroupId {
                                fn get(t: &script::trainer::Trainer) -> Option<TrainerGroupId> {
                                    let text = t.pic.split_once("TRAINER_PIC_").map(|(.., r)| r)?;
                                    let split = text.split_once('_')?;
                                    if split.0.eq_ignore_ascii_case("RS") {
                                        return None;
                                    }
                                    Some(text.to_ascii_lowercase().parse().ok()?)
                                }

                                get(t).unwrap_or_else(|| "placeholder".parse().unwrap())
                            }

                            trainer = Some(NpcTrainer {
                                group: get_group(t),
                                character: Trainer {
                                    party: party
                                        .iter()
                                        .flat_map(|p| {
                                            let id = p.species[8..].replace('_', "-");
                                            data.pokedex
                                                .try_get_named(&id)
                                                .map(|pokemon| {
                                                    let mut saved = SavedPokemon {
                                                        pokemon: pokemon.id,
                                                        level: p.level,
                                                        gender: None,
                                                        ivs: StatSet::uniform(p.ivs / 6),
                                                        ..Default::default()
                                                    };
                                                    if let Some(item) = &p.item {
                                                        let id = item[5..].replace('_', " ");
                                                        if let Some(item) = data
                                                            .itemdex
                                                            .try_get_named(&id)
                                                            .or_else(|| {
                                                                println!(
                                                                    "Cannot get item id {}",
                                                                    id
                                                                );
                                                                None
                                                            })
                                                        {
                                                            saved.item = Some(item.id);
                                                        }
                                                    }
                                                    if let Some(moves) = p.moves.as_ref() {
                                                        for m in moves {
                                                            let id = m[5..].replace('_', " ");
                                                            if let Some(m) = data
                                                                .movedex
                                                                .try_get_named(&id)
                                                                .or_else(|| {
                                                                    if !id.eq_ignore_ascii_case(
                                                                        "NONE",
                                                                    ) {
                                                                        println!(
                                                                            "Cannot get move id {}",
                                                                            id
                                                                        );
                                                                    }
                                                                    None
                                                                })
                                                            {
                                                                saved
                                                                    .moves
                                                                    .push(SavedMove::from(m.id));
                                                            }
                                                        }
                                                    }
                                                    saved
                                                })
                                                .or_else(|| {
                                                    println!("Cannot get pokemon id {}", id);
                                                    None
                                                })
                                        })
                                        .collect(),
                                    bag: Default::default(), //trainer.items.in,
                                    money: 0,
                                },
                                sight: match sight == 0 {
                                    true => None,
                                    false => Some(sight),
                                },
                                encounter: data.messages.get(encounter_id).unwrap().clone(),
                                defeat: data.messages.get(defeat_id).unwrap().clone(),
                                badge: None,
                                disable: TrainerDisable::DisableSelf,
                            });

                            // if let Some(post) = script
                            //     .commands
                            //     .iter()
                            //     .find(|command| command.command == "msgbox")
                            // {
                            //     let id = &post.arguments[0];
                            //     let message = data.messages.get(id).unwrap();
                            //     let message = message.value();
                            //     interact = NpcInteract::Message(message.clone());
                            // }
                        }
                    }

                    script_name = Some(script.name.clone());
                }

                if name.is_empty() {
                    name = format!("NPC {}-{}", event.x, event.y);
                }

                if interact.is_none() && !event.script.is_empty() {
                    interact = Some(event.script.clone());
                }

                let id = index as ObjectId;

                let group = group.parse().unwrap();
                Some((
                    (
                        id,
                        Npc {
                            id,
                            name,
                            origin: Position {
                                coords: Coordinate {
                                    x: event.x as _,
                                    y: event.y as _,
                                },
                                direction: *directions.iter().next().unwrap_or(&Direction::Down),
                                elevation: Elevation(event.elevation),
                            },
                            group,
                            movement: match movement {
                                true => {
                                    let empty = directions.len() <= 1;
                                    let mut vec = Vec::with_capacity(1 + if empty { 0 } else { 1 });
                                    vec.push(NpcMovement::Move(Coordinate {
                                        x: event.movement_range_x as _,
                                        y: event.movement_range_y as _,
                                    }));
                                    if !empty {
                                        vec.push(NpcMovement::Look(directions));
                                    }
                                    vec
                                }
                                false => match directions.len() <= 1 {
                                    true => Vec::new(),
                                    false => vec![NpcMovement::Look(directions)],
                                },
                            },
                            trainer,
                        },
                    ),
                    (id, script_name),
                ))
            } else {
                None
            }
        })
        .unzip();
    let b: HashMap<ObjectId, Option<String>> = b;
    (
        a,
        b.into_par_iter()
            .flat_map(|(a, b)| b.map(|b| (a, b)))
            .collect(),
    )
}

// fn into_world_objects(mappings: &NameMappings, events: &[JsonObjectEvent]) -> Objects {
//     events
//         .par_iter()
//         .enumerate()
//         .flat_map(
//             |(index, event)| match mappings.objects.objects.get(&event.graphics_id) {
//                 Some(id) => Some({
//                     (
//                         index as _,
//                         ObjectEntity {
//                             data: ObjectEntityData { group: *id },
//                             coordinate: Coordinate {
//                                 x: event.x as _,
//                                 y: event.y as _,
//                             },
//                         },
//                     )
//                 }),
//                 None => None,
//             },
//         )
//         .collect()
// }

// fn into_world_items(data: &ParsedData, events: &[JsonBgEvent]) -> Items {
//     events
//         .par_iter()
//         .enumerate()
//         .filter(|(_, event)| event.type_ == "hidden_item")
//         .flat_map(|(index, event)| {
//             Some((
//                 index as _,
//                 ItemEntity {
//                     data: ItemEntityData {
//                         item: ItemStack {
//                             item: {
//                                 let id = event.item.as_ref()?[5..]
//                                     .to_ascii_lowercase()
//                                     .parse()
//                                     .ok()?;
//                                 firecore_world_builder::world::pokedex::Dex::try_get(
//                                     &data.itemdex,
//                                     &id,
//                                 )
//                                 .or_else(|| {
//                                     if !id.eq_ignore_ascii_case("NONE") {
//                                         println!(
//                                             "Cannot get item id {} for hidden item",
//                                             id.as_str()
//                                         );
//                                     }
//                                     None
//                                 })?
//                                 .id
//                             },
//                             count: event.quantity?,
//                         },
//                         hidden: event.underfoot?,
//                     },
//                     coordinate: Coordinate {
//                         x: event.x as _,
//                         y: event.y as _,
//                     },
//                 },
//             ))
//         })
//         .collect()
// }

// fn into_world_signs(data: &ParsedData, events: &[JsonBgEvent]) -> Signs {
//     events
//         .par_iter()
//         .enumerate()
//         .filter(|(.., event)| event.type_ == "sign")
//         .flat_map(|(index, event)| {
//             let script = data.scripts.get(event.script.as_ref()?)?;
//             let msgbox = script
//                 .commands
//                 .iter()
//                 .find(|command| command.command == "msgbox")?;
//             let id = msgbox.arguments.get(0)?;
//             let message = data.messages.get(id)?.clone();
//             Some((
//                 index as _,
//                 SignEntity {
//                     data: SignEntityData { message },
//                     coordinate: Coordinate {
//                         x: event.x as _,
//                         y: event.y as _,
//                     },
//                 },
//             ))
//         })
//         .collect()
// }

fn into_palettes(mappings: &NameMappings, primary: &str, secondary: &str) -> [PaletteId; 2] {
    let primary = mappings
        .palettes
        .primary
        .get(primary)
        .copied()
        .unwrap_or_else(|| {
            eprintln!("Unknown primary tileset {}", primary);
            0
        });
    let secondary = mappings
        .palettes
        .secondary
        .get(secondary)
        .copied()
        .unwrap_or_else(|| {
            eprintln!("Unknown secondary tileset {}", secondary);
            13
        });

    [primary, secondary]
}

fn into_music(mappings: &NameMappings, music: &str) -> TinyStr16 {
    mappings.music.get(music).copied().unwrap_or_else(|| {
        eprintln!("Cannot find music {}", music);
        "pallet".parse().unwrap()
    })
}

// #[derive(Debug, Deserialize, Default)]
// #[serde(from = "String")]
// pub struct JsonMovementType(pub NpcMovement, pub Direction);

// impl From<String> for JsonMovementType {
//     fn from(string: String) -> Self {
//         match string.as_str() {

//             _ => Default::default(),
//         }
//     }
// }

// impl JsonMap {
//     pub fn save(self) {
//         let path = std::path::Path::new(&self.name);

//         std::fs::create_dir_all(&path).unwrap();

//         let npcs = path.join("npcs");

//         std::fs::create_dir_all(&npcs).unwrap();

//         for (index, event) in self.object_events.into_iter().enumerate() {
//             match event {
//                 object_events::MapObjectType::Npc(npc) => {
//                     let npc = SerializedNpc {
//                         id: {
//                             let t = format!("npc_{}", index);
//                             t.parse::<NpcId>().unwrap()
//                         },
//                         npc: npc,
//                     };
//                     let data = ron::ser::to_string_pretty(&npc, Default::default())
//                         .unwrap()
//                         .into_bytes();
//                     std::fs::write(npcs.join(format!("{}.ron", &npc.npc.character.name)), data)
//                         .unwrap();
//                 }
//                 object_events::MapObjectType::Other => (),
//             }
//         }
//     }
// }
