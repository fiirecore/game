use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::{
    deps::Random,
    util::{Entity, Completable, Direction, Timer, Location},
    pokedex::item::ItemStack,
    input::{pressed, Control, debug_pressed, DebugBind},
    text::MessagePage,
    storage::{
        data, data_mut,
        player::PlayerSave,
    },
    battle_glue::BattleEntryRef,
    play_music_named, play_music, play_sound,
    graphics::{position, draw_cursor},
    tetra::Context,
    log::{info, warn},
    audio,
    is_debug,
};

use worldlib::{
    character::{
        movement::MovementType,
        npc::{NPC, NPCId, NPCInteract},
        player::PlayerCharacter,
    },
    map::{
        TileId,
        World,
        WorldMap,
        manager::{can_move, WorldMapManagerData},
        ActiveNPC,
    },
    script::world::{WorldScript, Condition, WorldActionKind, ScriptWarp},
};

use crate::world::{
    GameWorld,
    WorldTextures,
    RenderCoords,
    gui::TextWindow,
    battle::{wild_battle, trainer_battle},
    npc::npc_type,
};

pub mod manager;

pub mod texture;
pub mod warp;

pub static NPC_RANDOM: Random = Random::new();
pub static mut NPC_TIMER: Timer = Timer::new(true, 0.5);
pub static WILD_ENCOUNTERS: AtomicBool = AtomicBool::new(true);

const NPC_MOVE_CHANCE: f32 = 1.0 / 12.0;

impl GameWorld for WorldMap {

    fn on_start(&mut self, ctx: &mut Context, music: bool) {

        // if let Some(saves) = get::<PlayerSaves>() {
        //     if let Some(data) = saves.get().world.map.get(&self.name) {
        //         for (index, state) in data.npcs.iter() {
        //             if let Some(npc) = self.npc_manager.npcs.get_mut(index) {
        //                 // npc.alive = *state;
        //             }
        //         }
        //     }
        // }

        if music {
            if audio::get_current_music().map(|current| current != self.music).unwrap_or(true) {
                play_music(ctx, self.music);
            }
        }

    }

    fn on_tile(&mut self, world: &mut WorldMapManagerData, battle: BattleEntryRef) {
        if let Some(tile_id) = self.tile(world.player.character.position.coords) {

            if WILD_ENCOUNTERS.load(Relaxed) {
                if let Some(wild) = &self.wild {
                    if wild.should_generate() {
                        if let Some(tiles) = wild.tiles.as_ref() {
                            for tile in tiles.iter() {
                                if &tile_id == tile {
                                    wild_battle(battle, wild);
                                    break;
                                }
                            }
                        } else {
                            wild_battle(battle, wild);
                        }
                    }          
                }
            }

            let save = data_mut();
    
            // look for player
            for (index, npc) in self.npcs.list.iter_mut().filter(|(_, npc)| npc.as_ref().map(|npc| npc.trainer.is_some()).unwrap_or(false)) {
                find_battle(save, &self.id, *index, npc, &mut self.npcs.active, &mut world.player);
            }

            for script in self.scripts.iter_mut() {
                if !script.alive() && script.in_location(&world.player.character.position.coords) {
                    let mut break_script = false;
                    for condition in &script.conditions {
                        match condition {
                            Condition::Scripts(scripts) => {
                                for script_condition in scripts {
                                    if save.world.scripts.contains(&script_condition.identifier).ne(&script_condition.happened) {
                                        break_script = true;
                                    }  
                                }                          
                            },
                            Condition::Activate(direction) => {
                                if world.player.character.position.direction.ne(direction) {
                                    break_script = true;
                                }
                            }
                            Condition::PlayerHasPokemon(is_true) => {
                                if save.party.is_empty().eq(is_true) {
                                    break_script = true;
                                }
                            }
                        }
                        if break_script {
                            break;
                        }
                    }
                    if !break_script {
                        script.spawn();
                    }
                }
            }

        }
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, world: &mut WorldMapManagerData, battle: BattleEntryRef, window: &mut TextWindow) {

        if is_debug() {
            debug_input(ctx, self);
        }

        if pressed(ctx, Control::A) && self.npcs.active.is_none() {
            let pos = if self.tile(world.player.character.position.coords).map(|tile| match tile {
                0x298 | 0x2A5 => true,
                _ => false,
            }).unwrap_or_default() {
                world.player.character.position.in_direction(world.player.character.position.direction)
            } else {
                world.player.character.position
            };
            for (id, npc_opt) in self.npcs.list.iter_mut() {
                if let Some(npc) = npc_opt {
                    if npc.interact.is_some() || npc.trainer.is_some() {
                        if npc.interact_from(&pos) {
                            self.npcs.active = npc_opt.take().map(|npc| (*id, npc));
                        }
                    }
                }
            }
        }

        // Move NPCs

        for npc in self.npcs.list.values_mut().flatten().filter(|npc| npc.character.destination.is_some() && npc.movement != MovementType::Still) {
            npc.character.move_to_destination(delta);            
        }

        if unsafe{NPC_TIMER.finished()} {
            unsafe{NPC_TIMER.soft_reset();}
            let save = data_mut();
            for (index, npc_opt) in self.npcs.list.iter_mut() {
                if let Some(npc) = npc_opt {
                    if npc.character.destination.is_none() {
                        if NPC_RANDOM.gen_float() < NPC_MOVE_CHANCE {
                            match npc.movement {
                                MovementType::Still => (),
                                MovementType::LookAround => {
                                    npc.character.position.direction = Direction::DIRECTIONS[NPC_RANDOM.gen_range(0, 4)];
                                    find_battle(save, &self.id, *index, npc_opt, &mut self.npcs.active, &mut world.player);
                                },
                                MovementType::WalkUpAndDown(steps) => {
                                    let origin = npc.origin.get_or_insert(npc.character.position.coords);
                                    let direction =
                                        if npc.character.position.coords.y <= origin.y - steps {
                                            Direction::Down
                                        } else if npc.character.position.coords.y >= origin.y + steps {
                                            Direction::Up
                                        } else
                                    if NPC_RANDOM.gen_bool() {
                                        Direction::Down
                                    } else {
                                        Direction::Up
                                    };
                                    let coords = npc.character.position.coords.in_direction(direction);
                                    if can_move(npc.character.move_type, self.movements[npc.character.position.coords.x as usize + npc.character.position.coords.y as usize * self.width]) {
                                        npc.character.position.direction = direction;
                                        if !find_battle(save, &self.id, *index, npc_opt, &mut self.npcs.active, &mut world.player) {
                                            if coords.y != world.player.character.position.coords.y {
                                                npc_opt.as_mut().unwrap().character.go_to(coords);
                                            }
                                        }
                                    }
                                },
                            }
                        }  
                    }
                }
            }
        } else {
            unsafe{NPC_TIMER.update(delta);}
        }

        

        // Update scripts

        for script in self.scripts.iter_mut().filter(|script| script.alive()) {
            let mut pop = false;
            match script.actions.front() {
                Some(action) => {
                    match action {
                        WorldActionKind::PlayMusic(music) => {
                            play_music_named(ctx, &music);
                            pop = true;
                        },
                        WorldActionKind::PlayMapMusic => {
                            play_music(ctx, self.music);
                            pop = true;
                        },
                        WorldActionKind::PlaySound(sound) => {
                            play_sound(ctx, sound);
                            pop = true;
                        }
                        WorldActionKind::PlayerFreezeInput => {
                            world.player.input_frozen = true;
                            world.player.character.stop_move();
                            pop = true;
                        },
                        WorldActionKind::PlayerUnfreezeInput => {
                            world.player.input_frozen = false;
                            pop = true;
                        }
                        WorldActionKind::PlayerUnfreeze => {
                            world.player.character.frozen = false;
                            pop = true;
                        }
                        WorldActionKind::PlayerLook(direction) => {
                            world.player.character.position.direction = *direction;
                            pop = true;
                        }
                        WorldActionKind::PlayerMove(destination) => {
                            if world.player.character.destination.is_some() {
                                if world.player.character.move_to_destination(delta) {
                                    pop = true;
                                }
                            } else {
                                world.player.character.move_to(*destination);
                            }
                        }
                        WorldActionKind::PlayerGivePokemon(instance) => {
                            if let Err(err) = data_mut().party.try_push(instance.clone()) {
                                warn!("Could not add {} to player's party with error {}", instance.pokemon.value().name, err);
                            }
                            pop = true;
                        }
                        WorldActionKind::PlayerHealPokemon => {
                            for pokemon in data_mut().party.iter_mut() {
                                pokemon.heal();
                            }
                            pop = true;
                        }

                        WorldActionKind::PlayerGiveItem(item) => {
                            data_mut().bag.add_item(ItemStack::new(item, 1));
                            pop = true;
                        }

                        WorldActionKind::NPCAdd(id, npc) => {
                            if self.npcs.list.insert(*id, Some(npc.clone())).is_some() {
                                warn!("Replaced NPC with id {}!", id);
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCRemove(id) => {
                            if self.npcs.list.remove(id).is_none() {
                                warn!("Could not remove NPC with id {}!", id);
                            }
                            pop = true;
                        }

                        // WorldActionKind::NPCSpawn(id) => {
                        //     if let Some(npc) = self.npc_manager.get_mut(id) {
                        //         error!("no npc spawn");// npc.spawn();
                        //     } else {
                        //         warn!("Could not spawn NPC with id {}!", id);
                        //     }
                        //     pop = true;
                        // }
                        // WorldActionKind::NPCDespawn(id) => {
                        //     if let Some(npc) = self.npc_manager.get_mut(id) {
                        //         error!("no npc despawn");// npc.despawn();
                        //     } else {
                        //         warn!("Could not despawn NPC with id {}!", id);
                        //     }
                        //     pop = true;
                        // }


                        WorldActionKind::NPCLook(id, direction) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                npc.character.position.direction = *direction;                               
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCMove( id, pos ) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if npc.character.destination.is_some() {
                                    if npc.character.move_to_destination(delta) {
                                        pop = true;
                                    }
                                } else {
                                    npc.character.go_to(pos.coords);
                                }
                            } else {
                                warn!("NPC script tried to move an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        },

                        WorldActionKind::NPCLeadPlayer( id, pos ) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if npc.character.destination.is_some() {
                                    npc.character.move_to_destination(delta);
                                } else {
                                    if npc.character.position.coords != pos.coords {
                                        npc.character.go_to(pos.coords);
                                    }
                                }
                                if world.player.character.destination.is_some() {
                                    if world.player.character.move_to_destination(delta) {
                                        pop = true;
                                    }
                                } else {
                                    if world.player.character.position.coords.ne(&pos.coords) {
                                        world.player.character.destination = npc.character.destination.clone();
                                        if let Some(destination) = world.player.character.destination.as_mut() {
                                            destination.queued_movements.pop_back();
                                            destination.queued_movements.push_front(world.player.character.position.coords.towards(npc.character.position.coords))
                                        }
                                        // player.move_to(Destination::next_to(&player.character.position.local, pos.coords));
                                    }
                                }
                            } else {
                                warn!("NPC script tried to lead player with an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCMoveToPlayer(id) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if npc.character.destination.is_some() {
                                    if npc.character.move_to_destination(delta) {
                                        pop = true;
                                    }
                                } else {
                                    npc.character.go_next_to(world.player.character.position.coords)
                                }
                            } else {
                                warn!("NPC script tried to move to player with an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCInteract(id) => {
                            if let Some(npc) = self.npcs.list.get_mut(id) {
                                if let Some(npc_mut) = npc {
                                    if npc_mut.interact_from(&world.player.character.position) {
                                        self.npcs.active = Some((*id, npc.take().unwrap()));
                                    }
                                }
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCSay(id, pages) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if window.text.alive() {
                                    if !window.text.finished() {
                                        window.text.update(ctx, delta);
                                    } else {
                                        window.text.despawn();
                                        pop = true;
                                    }
                                } else {
                                    window.text.clear();
                                    window.text.set(pages.clone());
                                    window.text.color(npc_type(&npc.npc_type).text_color);
                                    window.text.process_messages(data()); 
                                    window.text.spawn();   
                                }
                            } else {
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCBattle(id) => {
                            if let Some(npc) = self.npcs.get(id) {
                                trainer_battle(battle, &mut world.battling, npc, &self.id, id);
                            }
                            pop = true;
                        }

                        WorldActionKind::Wait(time) => {
                            if script.timer.alive() {
                                script.timer.update(delta);
                                if script.timer.finished() {
                                    script.timer.despawn();
                                    pop = true;
                                }
                            } else {
                                script.timer.hard_reset();
                                script.timer.spawn();
                                script.timer.length = *time;
                                script.timer.update(delta);
                            }
                        },

                        WorldActionKind::Info(string) => {
                            info!("{}: {}", script.identifier, string);
                            pop = true;
                        }

                        WorldActionKind::DisplayText(message) => {
                            if window.text.alive() {
                                if !window.text.finished() {
                                    window.text.update(ctx, delta);
                                } else {
                                    window.text.despawn();
                                    pop = true;
                                }
                            } else {
                                window.text.clear();
                                window.text.set(message.pages.clone());
                                window.text.color(message.color);
                                window.text.process_messages(data()); 
                                window.text.spawn();   
                            }
                        },

                        WorldActionKind::Conditional { message, end_message, unfreeze } => {

                            /*
                            * 0 = first message (default)
                            * 1 = end message
                            * 2 or 3 = yes/no option and cursor pos
                            */

                            if script.option == 0 {
                                if window.text.alive() {
                                    if window.text.finished() {
                                        script.option = 2;
                                    } else {
                                        window.text.update(ctx, delta);
                                    }
                                } else {
                                    window.text.clear();
                                    window.text.spawn();
                                    window.text.set(message.pages.clone());
                                    window.text.color(message.color);
                                }
                            } else if script.option == 1 {

                                if end_message.is_some() {

                                    if window.text.finished() {
                                        window.text.despawn();
                                        if *unfreeze {
                                            world.player.unfreeze();
                                        }
                                        script.option = 0;
                                        despawn_script(script);
                                    } else {
                                        window.text.update(ctx, delta);
                                    }

                                } else {
                                    if *unfreeze {
                                        world.player.unfreeze();
                                    }
                                    script.option = 0;
                                    despawn_script(script);
                                }
                            } else {
                                if pressed(ctx, Control::A) {
                                    if script.option == 2 {
                                        script.option = 0;
                                        window.text.despawn();
                                        pop = true;
                                    } else if script.option == 3 {

                                        script.option = 1;
                                        if let Some(end_message) = end_message {
                                            window.text.clear();
                                            window.text.set(end_message.pages.clone());
                                            window.text.color(end_message.color);
                                        }

                                    }
                                } else if pressed(ctx, Control::B) {

                                    script.option = 1;
                                    if let Some(end_message) = end_message {
                                        window.text.clear();
                                        window.text.set(end_message.pages.clone());
                                        window.text.color(end_message.color);
                                    }

                                }
                                if pressed(ctx, Control::Up) && script.option == 3 {
                                    script.option = 2;
                                }
                                if pressed(ctx, Control::Down) && script.option == 2 {
                                    script.option = 3;
                                }
                            }
                        }

                        WorldActionKind::Warp(warp_type) => {
                            world.warp = Some(match warp_type {
                                ScriptWarp::Id(id) => {
                                    self.warps.get(id).unwrap_or_else(|| panic!("Could not get warp with id {} under script {} because it doesn't exist!", id, script.identifier)).destination
                                }
                                ScriptWarp::Dest(destination) => {
                                    *destination
                                }
                                ScriptWarp::KeepMusic(id) => {
                                    let mut warp = self.warps.get(id).unwrap_or_else(|| panic!("Could not get warp with id {} under script {} because it doesn't exist!", id, script.identifier)).destination;
                                    warp.transition.change_music = false;
                                    warp
                                }
                            });
                            world.player.character.destination = None; // fix so this is not necessary
                            despawn_script(script);
                        },
                    }
                }
                None => {
                    despawn_script(script);
                }
            }
            if pop {
                script.actions.pop_front();
            }
        }

        // Npc window manager code

        // #[deprecated(note = "rewrite active NPC code")]
        if let Some((id, npc)) = self.npcs.active.as_mut() {
            if window.text.alive() {
                if window.text.finished() {
                    trainer_battle(battle, &mut world.battling, npc, &self.id, id);
                    window.text.despawn();
                    let (id, npc) = self.npcs.active.take().unwrap();
                    self.npcs.list.insert(id, Some(npc));
                    world.player.unfreeze();
                } else {
                    window.text.update(ctx, delta);
                }
            } else {
                if npc.character.destination.is_some() {
                    npc.character.move_to_destination(delta);
                } else {
                    window.text.spawn();
                    world.player.freeze_input();
                    npc.character.destination = None;
    
                    let mut message_ran = false;
    
                    match &npc.interact {
                        NPCInteract::Message(pages) => {
                            window.text.set(pages.clone());
                            window.text.color(super::npc::npc_type(&npc.npc_type).text_color);
                            message_ran = true;
                        },
                        NPCInteract::Script(_) => todo!(),
                        NPCInteract::Nothing => (),
                    }
                    
                    if !data_mut().world.get_map(&self.id).battled.contains(id) {
                        if let Some(trainer) = npc.trainer.as_ref() {

                            if trainer.battle_on_interact {

                                let npc_type = super::npc::npc_type(&npc.npc_type);
                                if let Some(trainer_type) = npc_type.trainer.as_ref() {

                                    // Spawn text window
                                    window.text.set(
                                        trainer.encounter_message.iter().map(|message| {
                                            MessagePage::new(
                                                message.clone(),
                                                None,
                                            )
                                        }).collect()
                                    );
                                    window.text.color(npc_type.text_color);
                                    message_ran = true;

                                    // Play Trainer music

                                    if let Some(encounter_music) = trainer_type.music.as_ref() {
                                        if let Some(playing_music) = audio::get_current_music() {
                                            if let Some(music) = audio::get_music_id(encounter_music).flatten() {
                                                if playing_music != music {
                                                play_music(ctx, music)
                                                }
                                            }
                                        } else {
                                            play_music_named(ctx, encounter_music)
                                        }
                                    }
                                }
                            }
                        }   
                    }
    
                    world.player.character.position.direction = npc.character.position.direction.inverse();
                    if world.player.character.is_frozen() {
                        world.player.unfreeze();
                    }

                    if !message_ran {
                        window.text.despawn();
                        let (id, npc) = self.npcs.active.take().unwrap();
                        self.npcs.list.insert(id, Some(npc));
                    } else {
                        window.text.process_messages(data());
                    }

                } 
            }
        }
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, door: &Option<worldlib::map::manager::Door>, screen: &RenderCoords, border: bool) {
        let primary = textures.tiles.palettes.get(&self.palettes[0]).expect("Could not get primary palette for map!");
        let length = primary.height() as TileId;
        let secondary = textures.tiles.palettes.get(&self.palettes[1]).expect("Could not get secondary palette for map!");

        for yy in screen.top..screen.bottom {
            let y = yy - screen.offset.y;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            let row = (y as usize).saturating_mul(self.width);
            
            for xx in screen.left..screen.right {
                let x = xx - screen.offset.x;
                let render_x = (xx << 4) as f32 - screen.focus.x;

                if !(x < 0 || y < 0 || y >= self.height as _ || x >= self.width as _) {
                    let index = x as usize + row as usize;
                    let tile = self.tiles[index];
                    let (texture, tile) = if length > tile { (primary, tile) } else { (secondary, tile - length) };
                    textures.tiles.draw_tile(ctx, texture, tile, render_x, render_y);
                    if let Some(door) = door {
                        if door.position == index {
                            textures.tiles.draw_door(ctx, door, render_x, render_y);
                        }
                    }
                } else if border {
                    let tile = self.border[if x % 2 == 0 { //  x % 2 + if y % 2 == 0 { 0 } else { 2 }
                        if y % 2 == 0 { 0 } else { 2 }
                    } else {
                        if y % 2 == 0 { 1 } else { 3 }
                    }];
                    let (texture, tile) = if length > tile { (primary, tile) } else { (secondary, tile - length) };
                    textures.tiles.draw_tile(ctx, texture, tile, render_x, render_y);
                }
            }
        }
        for npc in self.npcs.list.values().flatten() {
            textures.npcs.draw(ctx, npc, &screen);
        }
        if let Some((_, npc)) = self.npcs.active.as_ref() {
            textures.npcs.draw(ctx, npc, &screen);
        }
        for script in self.scripts.iter() {
            if script.alive() {
                if let Some(action) = script.actions.front() {
                    match action {
                        WorldActionKind::Conditional{ .. } => {
                                if script.option > 1 {
                                    textures.gui.get(&texture::gui::GuiTexture::Condition).draw(ctx, position(162.0, 66.0));
                                    draw_cursor(ctx, 170.0, 77.0 + (script.option - 2) as f32 * 16.0);
                                }
                        }
                        _ => (),
                    }                    
                }
            }            
        }
    }
}

fn debug_input(ctx: &Context, map: &mut WorldMap) {
    if debug_pressed(ctx, DebugBind::F8) {
        for (index, npc) in map.npcs.list.iter().flat_map(|(id, npc)| npc.as_ref().map(|npc| (id, npc))) {
            info!("NPC {} (id: {}), is at {}, {}; looking {:?}", &npc.name, index, /*if npc.alive() {""} else {" (despawned)"},*/ &npc.character.position.coords.x, &npc.character.position.coords.y, &npc.character.position.direction);
        }
    }
}

pub fn despawn_script(script: &mut WorldScript) {
    data_mut().world.scripts.insert(script.identifier);
    script.despawn();
}

fn find_battle(save: &mut PlayerSave, map: &Location, id: NPCId, npc: &mut Option<NPC>, active: &mut ActiveNPC, player: &mut PlayerCharacter) -> bool {
    if !save.world.has_battled(map, &id) {
        if npc.as_mut().map(|npc| npc.find_character(&mut player.character)).unwrap_or_default() {
            *active = Some((id, npc.take().unwrap()));
            true
        } else {
            false
        }
    } else {
        false
    }
}