use engine::{
    audio::{play_music, play_music_named, play_sound},
    util::{Completable, Entity},
    EngineContext,
};
use log::warn;
use pokedex::{Dex, context::PokedexClientContext, item::ItemStack, pokemon::owned::OwnedPokemon};
use saves::PlayerData;
use worldlib::{
    character::Movement,
    map::{manager::WorldMapData, WorldMap},
    positions::{Coordinate, Destination},
    script::world::{NpcWarp, PlayerWarp, WorldAction},
};

use crate::{
    game::battle_glue::BattleEntryRef,
    world::{gui::TextWindow, map::warp::WarpTransition},
};

/// Update scripts from WorldManager
pub(crate) fn update_script<'d>(
    ctx: &mut EngineContext,
    dex: &PokedexClientContext<'d>,
    save: &mut PlayerData<'d>,
    delta: f32,
    map: &mut WorldMap,
    world: &mut WorldMapData,
    battle: BattleEntryRef,
    window: &mut TextWindow,
    warper: &mut WarpTransition,
) {
    if let Some(action) = world.script.actions.last_mut() {
        match action {
            WorldAction::PlayMusic(music) => {
                play_music_named(ctx, music);
                world.script.actions.pop();
            }
            WorldAction::PlayMapMusic => {
                play_music(ctx, map.music);
                world.script.actions.pop();
            }
            WorldAction::PlaySound(sound) => {
                play_sound(ctx, sound);
                world.script.actions.pop();
            }
            WorldAction::PlayerFreezeInput => {
                world.player.input_frozen = true;
                world.player.stop_move();
                world.script.actions.pop();
            }
            WorldAction::PlayerUnfreezeInput => {
                world.player.input_frozen = false;
                world.script.actions.pop();
            }
            WorldAction::PlayerLook(direction) => {
                world.player.position.direction = *direction;
                world.script.actions.pop();
            }
            WorldAction::PlayerMove(x, y) => {
                if !world.player.moving() {
                    match world.player.character.position.coords.equal(x, y) {
                        true => {
                            world.script.actions.pop();
                        }
                        false => {
                            let destination = Destination::to(
                                &world.player.character.position,
                                Coordinate::new(*x, *y),
                            );
                            world.player.pathfind(destination);
                        }
                    }
                }
                // warn!("Waiting on player move");
            }
            WorldAction::PlayerGivePokemon(instance) => {
                match save.party.is_full() {
                    false => match instance.clone().init(
                        &mut rand::thread_rng(),
                        dex.pokedex,
                        dex.movedex,
                        dex.itemdex,
                    ) {
                        Some(p) => save.party.push(p),
                        None => warn!("Cannot initialize given pokemon!"),
                    },
                    true => warn!("PlayerGivePokemon command requires party space"),
                }
                world.script.actions.pop();
            }
            WorldAction::PlayerHealPokemon => {
                save.party.iter_mut().for_each(|o| o.heal(None, None));
                world.script.actions.pop();
            }
            WorldAction::PlayerGiveItem(item) => {
                match dex.itemdex.try_get(item) {
                    Some(item) => {
                        save.bag.add_item(ItemStack::new(item, 1));
                    },
                    None => warn!("Could not get item {}", item),
                }
                world.script.actions.pop();
            }
            WorldAction::NpcAdd(id, npc) => {
                if world
                    .script
                    .npcs
                    .insert(*id, (map.id, *npc.clone()))
                    .is_some()
                {
                    warn!("Replaced Npc with id {}!", id);
                }
                world.script.actions.pop();
            }
            WorldAction::NpcRemove(id) => {
                if world.script.npcs.remove(id).is_none() {
                    warn!("Could not remove Npc with id {}!", id);
                }
                world.script.actions.pop();
            }
            WorldAction::NpcLook(id, direction) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    npc.character.position.direction = *direction;
                }
                world.script.actions.pop();
            }
            WorldAction::NpcMove(id, x, y) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    if !npc.character.moving() {
                        match npc.character.position.coords.equal(x, y) {
                            true => {
                                world.script.actions.pop();
                            }
                            false => npc.character.pathfind(Destination::to(
                                &npc.character.position,
                                Coordinate::new(*x, *y),
                            )),
                        }
                    }
                } else {
                    warn!("Npc script tried to move an unknown Npc (with id {})", id);
                    world.script.actions.pop();
                }
            }
            WorldAction::NpcLeadPlayer(id, x, y) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    if !world.player.moving() {
                        match world.player.character.position.coords.equal(x, y) {
                            true => {
                                world.player.input_frozen = false;
                                world.script.actions.pop();
                            }
                            false => {
                                if !npc.character.position.coords.equal(x, y) {
                                    npc.character.pathfind(Destination::to(
                                        &npc.character.position,
                                        Coordinate::new(*x, *y),
                                    ));
                                }

                                world.player.input_frozen = true;
                                world.player.movement = Movement::Walking;

                                if !world.player.position.coords.equal(x, y) {
                                    world.player.pathing.queue =
                                        npc.character.pathing.queue.clone();

                                    world.player.pathing.queue.pop();

                                    let d = world
                                        .player
                                        .position
                                        .coords
                                        .towards(npc.character.position.coords);

                                    world.player.pathing.queue.insert(0, d);

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
                    world.script.actions.pop();
                }
            }
            WorldAction::NpcMoveToPlayer(id) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    if !npc.character.moving() {
                        match npc
                            .character
                            .position
                            .coords
                            .in_direction(npc.character.position.direction)
                            == world.player.position.coords
                        {
                            true => {
                                world.script.actions.pop();
                            }
                            false => npc.character.pathfind(Destination::next_to(
                                &npc.character.position,
                                world.player.position.coords,
                            )),
                        }
                    }
                } else {
                    warn!(
                        "Npc script tried to move to player with an unknown Npc (with id {})",
                        id
                    );
                    world.script.actions.pop();
                }
            }
            WorldAction::NpcInteract(id) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    if npc.interact_from(&world.player.position) {
                        world.npc.active = Some(*id);
                    }
                }
                world.script.actions.pop();
            }
            WorldAction::NpcSay(id, pages) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    match window.text.alive() {
                        true => match window.text.finished() {
                            false => window.text.update(ctx, delta),
                            true => {
                                window.text.despawn();
                                world.script.actions.pop();
                            }
                        },
                        false => {
                            window.text.clear();
                            let mut pages = pages.clone();
                            crate::game::text::process_messages(&mut pages, save);
                            window.text.set(pages);
                            window
                                .text
                                .color(crate::world::npc::npc_type(&npc.type_id).text_color);
                            window.text.spawn();
                        }
                    }
                } else {
                    world.script.actions.pop();
                }
            }
            WorldAction::NpcBattle(id) => {
                if let Some(npc) = get_npc(id, &mut world.script.npcs, &mut map.npcs) {
                    crate::world::battle::trainer_battle(
                        save,
                        battle,
                        &mut world.battling,
                        npc,
                        &map.id,
                        id,
                    );
                }
                world.script.actions.pop();
            }
            WorldAction::NpcWarp(id, warp) => {
                match world.script.npcs.get_mut(id) {
                    Some((npc_location, npc)) => {
                        if let Some((location, destination)) = match warp {
                            NpcWarp::Id(id) => match map.warps.get(id) {
                                Some(warp) => {
                                    Some((warp.destination.location, warp.destination.position))
                                }
                                None => {
                                    log::error!("Could not get warp {} in map {}", id, map.name);
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
                world.script.actions.pop();
            }
            // WorldAction::Info(text) => {
            //     info!("{}: {}", script.identifier, string);
            //     world.script.actions.pop();
            // },
            WorldAction::Wait(remaining) => {
                *remaining -= delta;
                if remaining < &mut 0.0 {
                    world.script.actions.pop();
                }
            }
            WorldAction::WaitFinishWarp => {
                if !warper.alive() {
                    world.script.actions.pop();
                }
            }
            WorldAction::DisplayText(message) => match window.text.alive() {
                false => {
                    window.text.clear();
                    let mut pages = message.pages.clone();
                    crate::game::text::process_messages(&mut pages, save);
                    window.text.set(pages);
                    window.text.color(message.color);
                    window.text.spawn();
                }
                true => {
                    if !window.text.finished() {
                        window.text.update(ctx, delta);
                    } else {
                        window.text.despawn();
                        world.script.actions.pop();
                    }
                }
            },
            WorldAction::Conditional { .. } => {
                log::error!("cannot use script Conditional command");
                world.script.actions.pop();
            }
            WorldAction::Warp(warp, music) => {
                let mut destination = match warp {
                    PlayerWarp::Id(id) => {
                        map.warps
                            .get(id)
                            .unwrap_or_else(|| {
                                log::error!(
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
                world.warp = Some(destination);
            }
            WorldAction::Finish(id) => {
                save.world.scripts.insert(*id);
                world.script.actions.pop();
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
    //                                 world.player.unfreeze();
    //                             }
    //                             script.option = 0;
    //                             despawn_script(script);
    //                         } else {
    //                             window.text.update(ctx, delta);
    //                         }
    //                     } else {
    //                         if *unfreeze {
    //                             world.player.unfreeze();
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

use hashbrown::HashMap;
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
