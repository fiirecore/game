use crate::{
    engine::{
        audio::{play_music, play_sound},
        log::{error, warn},
        util::{Completable, Entity},
        Context,
    },
    Sender, world::npc::color,
};
use firecore_battle_gui::pokedex::engine::gui::MessagePage;
use worldlib::{
    character::{player::PlayerCharacter, Movement, npc::MessageColor},
    map::WorldMap,
    positions::{Coordinate, Destination},
    script::world::{NpcWarp, PlayerWarp, WorldAction},
};

use crate::world::{gui::TextWindow, map::warp::WarpTransition, npc::NpcTypes, WorldActions};

/// Update scripts from WorldManager
pub(crate) fn update_script<'d>(
    ctx: &mut Context,
    // save: &mut PlayerData<'d>,
    delta: f32,
    map: &mut WorldMap,
    player: &mut PlayerCharacter,
    sender: &Sender<WorldActions>,
    npc_types: &NpcTypes,
    window: &mut TextWindow,
    warper: &mut WarpTransition,
) {
    if let Some(action) = player.world.scripts.actions.last_mut() {
        match action {
            WorldAction::PlayMusic(music) => {
                play_music(ctx, music);
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayMapMusic => {
                play_music(ctx, &map.music);
                player.world.scripts.actions.pop();
            }
            WorldAction::PlaySound(sound, variant) => {
                play_sound(ctx, sound, *variant);
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayerFreezeInput => {
                player.input_frozen = true;
                player.stop_move();
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayerUnfreezeInput => {
                player.input_frozen = false;
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayerLook(direction) => {
                player.position.direction = *direction;
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayerMove(x, y) => {
                if !player.character.moving() {
                    match player.character.position.coords.equal(x, y) {
                        true => {
                            player.world.scripts.actions.pop();
                        }
                        false => {
                            let destination = Destination::to(
                                &player.character.position,
                                Coordinate::new(*x, *y),
                            );
                            player.pathfind(destination);
                        }
                    }
                }
                // warn!("Waiting on player move");
            }
            WorldAction::PlayerGivePokemon(..) => {
                if let Some(WorldAction::PlayerGivePokemon(pokemon)) =
                    player.world.scripts.actions.pop()
                {
                    sender.send(WorldActions::GivePokemon(pokemon));
                }
                // match save.party.is_full() {
                //     false => match instance.clone().init(
                //         random,
                //         pokedex,
                //         movedex,
                //         itemdex,
                //     ) {
                //         Some(p) => save.party.push(p),
                //         None => warn!("Cannot initialize given pokemon!"),
                //     },
                //     true => warn!("PlayerGivePokemon command requires party space"),
                // }
            }
            WorldAction::PlayerHealPokemon => {
                sender.send(WorldActions::HealPokemon(None));
                player.world.scripts.actions.pop();
            }
            WorldAction::PlayerGiveItem(..) => {
                if let Some(WorldAction::PlayerGiveItem(item)) = player.world.scripts.actions.pop() {
                    sender.send(WorldActions::GiveItem(item));
                }
            }
            WorldAction::NpcAdd(id, npc) => {
                if player
                    .world
                    .scripts
                    .npcs
                    .insert(*id, (map.id, *npc.clone()))
                    .is_some()
                {
                    warn!("Replaced Npc with id {}!", id);
                }
                player.world.scripts.actions.pop();
            }
            WorldAction::NpcRemove(id) => {
                if player.world.scripts.npcs.remove(id).is_none() {
                    warn!("Could not remove Npc with id {}!", id);
                }
                player.world.scripts.actions.pop();
            }
            WorldAction::NpcLook(id, direction) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    npc.character.position.direction = *direction;
                }
                player.world.scripts.actions.pop();
            }
            WorldAction::NpcMove(id, x, y) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    if !npc.character.moving() {
                        match npc.character.position.coords.equal(x, y) {
                            true => {
                                player.world.scripts.actions.pop();
                            }
                            false => npc.character.pathfind(Destination::to(
                                &npc.character.position,
                                Coordinate::new(*x, *y),
                            )),
                        }
                    }
                } else {
                    warn!("Npc script tried to move an unknown Npc (with id {})", id);
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::NpcLeadPlayer(id, x, y) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    if !player.character.moving() {
                        match player.character.position.coords.equal(x, y) {
                            true => {
                                player.input_frozen = false;
                                player.world.scripts.actions.pop();
                            }
                            false => {
                                if !npc.character.position.coords.equal(x, y) {
                                    npc.character.pathfind(Destination::to(
                                        &npc.character.position,
                                        Coordinate::new(*x, *y),
                                    ));
                                }

                                player.input_frozen = true;
                                player.character.movement = Movement::Walking;

                                if !player.character.position.coords.equal(x, y) {
                                    player.character.pathing.queue = npc.character.pathing.queue.clone();

                                    player.character.pathing.queue.pop();

                                    let d = player.character
                                        .position
                                        .coords
                                        .towards(npc.character.position.coords);

                                    player.pathing.queue.insert(0, d);

                                    // player.move_to(Destination::next_to(&player.position.local, pos.coords));
                                }
                            }
                        }
                    }
                } else {
                    warn!(
                        "Npc script tried to lead player with an unknown Npc (with id {})",
                        id
                    );
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::NpcMoveToPlayer(id) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    if !npc.character.moving() {
                        match npc
                            .character
                            .position
                            .coords
                            .in_direction(npc.character.position.direction)
                            == player.character.position.coords
                        {
                            true => {
                                player.world.scripts.actions.pop();
                            }
                            false => npc.character.pathfind(Destination::next_to(
                                &npc.character.position,
                                player.character.position.coords,
                            )),
                        }
                    }
                } else {
                    warn!(
                        "Npc script tried to move to player with an unknown Npc (with id {})",
                        id
                    );
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::NpcInteract(id) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    if npc.interact_from(&player.character.position) {
                        player.world.npc.active = Some(*id);
                    }
                }
                player.world.scripts.actions.pop();
            }
            WorldAction::NpcSay(id, pages, queue) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    let color = color(npc_types.get(&npc.type_id).map(|npc| &npc.message).unwrap_or(&MessageColor::Black));
                    let pages = std::mem::take(pages);
                    
                    sender.send(WorldActions::Message(
                        pages.into_iter().map(MessagePage::from).collect(),
                        Some(color),
                        *queue,
                    ));
                } else {
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::NpcBattle(id) => {
                if let Some(npc) = get_npc(id, &mut player.world.scripts.npcs, &mut map.npcs) {
                    if let Some(entry) = crate::world::battle::trainer_battle(
                        npc_types,
                        &mut player.world.battle,
                        &map.id,
                        id,
                        npc,
                    ) {
                        sender.send(WorldActions::Battle(entry));
                    }
                }
                player.world.scripts.actions.pop();
            }
            WorldAction::NpcWarp(id, warp) => {
                match player.world.scripts.npcs.get_mut(id) {
                    Some((npc_location, npc)) => {
                        if let Some((location, destination)) = match warp {
                            NpcWarp::Id(id) => match map.warps.get(id) {
                                Some(warp) => {
                                    Some((warp.destination.location, warp.destination.position))
                                }
                                None => {
                                    error!("Could not get warp {} in map {}", id, map.name);
                                    None
                                }
                            },
                            NpcWarp::Dest(location, position) => {
                                Some((*location, Destination::from(*position)))
                            }
                        } {
                            *npc_location = location;
                            npc.character.position.from_destination(destination);
                        }
                    }
                    None => warn!("Could not get script Npc with id {}", id),
                }
                player.world.scripts.actions.pop();
            }
            // WorldAction::Info(text) => {
            //     info!("{}: {}", script.identifier, string);
            //     player.world.scripts.actions.pop();
            // },
            WorldAction::Wait(remaining) => {
                *remaining -= delta;
                if remaining < &mut 0.0 {
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::WaitMessage => {
                if window.text.finished() || !window.text.alive() {
                    window.text.despawn();
                    player.world.scripts.actions.pop();
                }
            }
            WorldAction::WaitFinishWarp => {
                if !warper.alive() {
                    player.world.scripts.actions.pop();
                }
            }
            // WorldAction::DisplayText(message) => match window.text.alive() {
            //     false => {
            //         let mut pages = message.pages.clone();
            //     }
            //     true => {
            //         if !window.text.finished() {
            //             window.text.update(ctx, delta);
            //         } else {
            //             window.text.despawn();
            //             player.world.scripts.actions.pop();
            //         }
            //     }
            // },
            // WorldAction::Conditional { .. } => {
            //     error!("cannot use script Conditional command");
            //     player.world.scripts.actions.pop();
            // }
            WorldAction::Warp(warp, music) => {
                let mut destination = match warp {
                    PlayerWarp::Id(id) => {
                        map.warps
                            .get(id)
                            .unwrap_or_else(|| {
                                error!(
                                "Could not get warp with id {} in map {} because it doesn't exist!",
                                id,
                                map.name,
                            );
                                panic!("Available warps: {:?}", map.warps.keys())
                            })
                            .destination
                    }
                    PlayerWarp::Dest(destination) => *destination,
                };
                destination.transition.change_music = *music;
                player.world.warp = Some(destination);
            }
            WorldAction::Finish(id) => {
                sender.send(WorldActions::EndScript(*id));
                player.world.scripts.actions.pop();
            }
        }
    }

    // for script in map.scripts.iter_mut().filter(|script| script.alive()) {
    //     match &mut script.current {
    //         None => match script.actions.pop_front() {
    //             Some(action) => {
    //                 if match &action {

    //                     WorldActionKind::Conditional { message, .. } => {
    //                         script.option = 0;
    //                         window.text.clear();
    //                         window.text.spawn();
    //                         window.text.set(message.pages.clone());
    //                         window.text.color(message.color);
    //                         true
    //                     }
    //                 } {
    //                     script.current = Some(action);
    //                 }
    //             }

    //             None => {
    //                 despawn_script(script);
    //             }
    //         },
    //         Some(current) => match current {
    //             WorldActionKind::Conditional {
    //                 end_message,
    //                 unfreeze,
    //                 ..
    //             } => {
    //                 /*
    //                  * 0 = first message (default)
    //                  * 1 = end message
    //                  * 2 or 3 = yes/no option and cursor pos
    //                  */
    //                 if script.option == 0 {
    //                     if window.text.finished() {
    //                         script.option = 2;
    //                     } else {
    //                         window.text.update(ctx, delta);
    //                     }
    //                 } else if script.option == 1 {
    //                     if end_message.is_some() {
    //                         if window.text.finished() {
    //                             window.text.despawn();
    //                             if *unfreeze {
    //                                  player.unfreeze();
    //                             }
    //                             script.option = 0;
    //                             despawn_script(script);
    //                         } else {
    //                             window.text.update(ctx, delta);
    //                         }
    //                     } else {
    //                         if *unfreeze {
    //                              player.unfreeze();
    //                         }
    //                         script.option = 0;
    //                         despawn_script(script);
    //                     }
    //                 } else {
    //                     if pressed(ctx, Control::A) {
    //                         if script.option == 2 {
    //                             script.option = 0;
    //                             window.text.despawn();
    //                             script.current = None;
    //                         } else if script.option == 3 {
    //                             script.option = 1;
    //                             if let Some(end_message) = end_message {
    //                                 window.text.clear();
    //                                 window.text.set(end_message.pages.clone());
    //                                 window.text.color(end_message.color);
    //                             }
    //                         }
    //                     } else if pressed(ctx, Control::B) {
    //                         script.option = 1;
    //                         if let Some(end_message) = end_message {
    //                             window.text.clear();
    //                             window.text.set(end_message.pages.clone());
    //                             window.text.color(end_message.color);
    //                         }
    //                     }
    //                     if pressed(ctx, Control::Up) && script.option == 3 {
    //                         script.option = 2;
    //                     }
    //                     if pressed(ctx, Control::Down) && script.option == 2 {
    //                         script.option = 3;
    //                     }
    //                 }
    //             }
    //             #[cfg(debug_assertions)]
    //             _ => {
    //                 warn!(
    //                     "Script {} tried to make action {:?} current when it cannot be.",
    //                     script.identifier, script.current
    //                 );
    //                 script.current = None;
    //             }
    //             #[cfg(not(debug_assertions))]
    //             _ => script.current = None,
    //         },
    //     }
    // }
}

use std::collections::HashMap;
use worldlib::{
    character::npc::{Npc, NpcId, Npcs},
    positions::Location,
};

fn get_npc<'a>(
    id: &NpcId,
    script_npcs: &'a mut HashMap<NpcId, (Location, Npc)>,
    map_npcs: &'a mut Npcs,
) -> Option<&'a mut Npc> {
    match script_npcs.get_mut(id) {
        Some((.., npc)) => Some(npc), //match location == map.id {}
        None => map_npcs.get_mut(id),
    }
}
