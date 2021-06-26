use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::{
    deps::random::{Random, RandomState, GLOBAL_STATE},
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
    tetra::{Context, graphics::Color},
    log::{info, warn},
    audio,
    is_debug,
};

use worldlib::{
    character::{
        movement::MovementType,
        npc::{Npc, NpcId, NpcInteract},
        player::PlayerCharacter,
    },
    map::{
        TileId,
        World,
        WorldMap,
        manager::{can_move, WorldMapManagerData},
        ActiveNpc,
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

pub static NPC_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));
pub static mut NPC_TIMER: Timer = Timer::new(true, 0.5);
pub static WILD_ENCOUNTERS: AtomicBool = AtomicBool::new(true);

const NPC_MOVE_CHANCE: f32 = 1.0 / 12.0;

impl GameWorld for WorldMap {

    fn on_start(&mut self, ctx: &mut Context, music: bool) {

        // if let Some(saves) = get::<PlayerSaves>() {
        //     if let Some(data) = saves.get().world.map.get(&self.name) {
        //         for (index, state) in data.npcs.iter() {
        //             if let Some(npc) = self.NPC_manager.npcs.get_mut(index) {
        //                 // npc.alive = *state;
        //             }
        //         }
        //     }
        // }

        if music {
            if audio::music::get_current_music().map(|current| current != self.music).unwrap_or(true) {
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
            let pos = if self.tile(world.player.character.position.coords).map(|tile| matches!(tile, 0x298 | 0x2A5)).unwrap_or_default() {
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

        // Move Npcs

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
            match &mut script.current {
                None => match script.actions.pop_front() {
                    Some(action) => if match &action {
                        WorldActionKind::PlayMusic(music) => {
                            play_music_named(ctx, music);
                            false
                        },
                        WorldActionKind::PlayMapMusic => {
                            play_music(ctx, self.music);
                            false
                        },
                        WorldActionKind::PlaySound(sound) => {
                            play_sound(ctx, sound);
                            false
                        }
                        WorldActionKind::PlayerFreezeInput => {
                            world.player.input_frozen = true;
                            world.player.character.stop_move();
                            false
                        },
                        WorldActionKind::PlayerUnfreezeInput => {
                            world.player.input_frozen = false;
                            false
                        }
                        WorldActionKind::PlayerUnfreeze => {
                            world.player.character.frozen = false;
                            false
                        }
                        WorldActionKind::PlayerLook(direction) => {
                            world.player.character.position.direction = *direction;
                            false
                        }
                        WorldActionKind::PlayerMove(destination) => {
                            world.player.character.move_to(*destination);
                            true
                        },
                        WorldActionKind::PlayerGivePokemon(instance) => {
                            if let Err(err) = data_mut().party.try_push(instance.clone()) {
                                warn!("Could not add {} to player's party with error {}", instance.pokemon.value().name, err);
                            }
                            false
                        }
                        WorldActionKind::PlayerHealPokemon => {
                            for pokemon in data_mut().party.iter_mut() {
                                pokemon.heal();
                            }
                            false
                        }

                        WorldActionKind::PlayerGiveItem(item) => {
                            data_mut().bag.add_item(ItemStack::new(item, 1));
                            false
                        }

                        WorldActionKind::NpcAdd(id, npc) => {
                            if self.npcs.list.insert(*id, Some(*npc.clone())).is_some() {
                                warn!("Replaced Npc with id {}!", id);
                            }
                            false
                        }

                        WorldActionKind::NpcRemove(id) => {
                            if self.npcs.list.remove(id).is_none() {
                                warn!("Could not remove Npc with id {}!", id);
                            }
                            false
                        }

                        WorldActionKind::NpcLook(id, direction) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                npc.character.position.direction = *direction;
                            }
                            false
                        }

                        WorldActionKind::NpcMove(id, pos) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                npc.character.go_to(pos.coords);
                                true
                            } else {
                                warn!("Npc script tried to move an unknown Npc (with id {})", id);
                                false
                            }
                        },

                        WorldActionKind::NpcLeadPlayer( id, pos ) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if npc.character.position.coords != pos.coords {
                                    npc.character.go_to(pos.coords);
                                }
                                if world.player.character.position.coords.ne(&pos.coords) {
                                    world.player.character.destination = npc.character.destination.clone();
                                    if let Some(destination) = world.player.character.destination.as_mut() {
                                        destination.queued_movements.pop_back();
                                        destination.queued_movements.push_front(world.player.character.position.coords.towards(npc.character.position.coords))
                                    }
                                    // player.move_to(Destination::next_to(&player.character.position.local, pos.coords));
                                }
                                true
                            } else {
                                warn!("Npc script tried to lead player with an unknown Npc (with id {})", id);
                                false
                            }
                        }

                        WorldActionKind::NpcMoveToPlayer(id) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                npc.character.go_next_to(world.player.character.position.coords);
                                true
                            } else {
                                warn!("Npc script tried to move to player with an unknown Npc (with id {})", id);
                                false
                            }
                        }

                        WorldActionKind::NpcInteract(id) => {
                            if let Some(npc) = self.npcs.list.get_mut(id) {
                                if let Some(npc_mut) = npc {
                                    if npc_mut.interact_from(&world.player.character.position) {
                                        self.npcs.active = Some((*id, npc.take().unwrap()));
                                    }
                                }
                            }
                            false
                        }

                        WorldActionKind::NpcSay(id, pages) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                window.text.clear();
                                window.text.set(pages.clone());
                                window.text.color(npc_type(&npc.type_id).text_color);
                                window.text.process_messages(data()); 
                                window.text.spawn();
                                true
                            } else {
                                false
                            }
                        }

                        WorldActionKind::NpcBattle(id) => {
                            if let Some(npc) = self.npcs.get(id) {
                                trainer_battle(battle, &mut world.battling, npc, &self.id, id);
                            }
                            false
                        }

                        WorldActionKind::Wait(time) => {
                            script.timer.hard_reset();
                            script.timer.spawn();
                            script.timer.length = *time;
                            script.timer.update(delta);
                            true
                        },

                        WorldActionKind::Info(string) => {
                            info!("{}: {}", script.identifier, string);
                            true
                        }

                        WorldActionKind::DisplayText(message) => {
                            window.text.clear();
                            window.text.set(message.pages.clone());
                            window.text.color(message.color);
                            window.text.process_messages(data()); 
                            window.text.spawn();
                            true
                        }

                        WorldActionKind::Conditional { message, .. } => {
                            script.option = 0;
                            window.text.clear();
                            window.text.spawn();
                            window.text.set(message.pages.clone());
                            window.text.color(message.color);
                            true
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
                            true
                        }

                    } {
                        script.current = Some(action);
                    }
                        
                    None => {
                        despawn_script(script);
                    }
                }
                Some(current) => match current {
                    WorldActionKind::PlayerMove(..) => {
                        if world.player.character.move_to_destination(delta) {
                            script.current = None;
                        }
                    },
                    WorldActionKind::NpcMove(id, ..) => {
                        match self.npcs.get_mut(id) {
                            Some(npc) => if npc.character.move_to_destination(delta) {
                                script.current = None;
                            }
                            None => script.current = None,
                        }
                    }
                    WorldActionKind::NpcLeadPlayer(id, ..) => {
                        match self.npcs.get_mut(id) {
                            Some(npc) => {
                                if npc.character.destination.is_some() {
                                    npc.character.move_to_destination(delta);
                                        npc.character.move_to_destination(delta);
                                }
                                if world.player.character.destination.is_some() {
                                    if world.player.character.move_to_destination(delta) {
                                        script.current = None;
                                    }
                                }
                            }
                            None => script.current = None,
                        }
                    }
                    WorldActionKind::NpcMoveToPlayer(id) => {
                        match self.npcs.get_mut(id) {
                            Some(npc) => if npc.character.move_to_destination(delta) {
                                script.current = None;
                            }
                            None => script.current = None,
                        }
                    }
                    WorldActionKind::NpcSay(..) => {
                        if !window.text.finished() {
                            window.text.update(ctx, delta);
                        } else {
                            window.text.despawn();
                            script.current = None;
                        }
                    },
                    WorldActionKind::Wait(..) => {
                        script.timer.update(delta);
                        if script.timer.finished() {
                            script.timer.despawn();
                            script.current = None;
                        }
                    }
                    WorldActionKind::DisplayText(..) => {
                        if !window.text.finished() {
                            window.text.update(ctx, delta);
                        } else {
                            window.text.despawn();
                            script.current = None;
                        }
                    }
                    WorldActionKind::Conditional{ end_message, unfreeze, .. } => {

                        /*
                        * 0 = first message (default)
                        * 1 = end message
                        * 2 or 3 = yes/no option and cursor pos
                        */

                        if script.option == 0 {
                            if window.text.finished() {
                                script.option = 2;
                            } else {
                                window.text.update(ctx, delta);
                            }
                        } else if script.option == 1 {
                            if end_message.is_some() {
                                if window.text.finished() {
                                    window.text.despawn();
                                    if *unfreeze {
                                        world.player.character.unfreeze();
                                    }
                                    script.option = 0;
                                    despawn_script(script);
                                } else {
                                    window.text.update(ctx, delta);
                                }
                            } else {
                                if *unfreeze {
                                    world.player.character.unfreeze();
                                }
                                script.option = 0;
                                despawn_script(script);
                            }
                        } else {
                            if pressed(ctx, Control::A) {
                                if script.option == 2 {
                                    script.option = 0;
                                    window.text.despawn();
                                    script.current = None;
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
                    #[cfg(debug_assertions)]
                    _ =>  {
                        warn!("Script {} tried to make action {:?} current when it cannot be.", script.identifier, script.current);
                        script.current = None;
                    }
                    #[cfg(not(debug_assertions))]
                    _ => script.current = None,
                },
            }
        }

        // Npc window manager code

        // #[deprecated(note = "rewrite active Npc code")]
        if let Some((id, npc)) = self.npcs.active.as_mut() {
            if window.text.alive() {
                if window.text.finished() {
                    trainer_battle(battle, &mut world.battling, npc, &self.id, id);
                    window.text.despawn();
                    let (id, npc) = self.npcs.active.take().unwrap();
                    self.npcs.list.insert(id, Some(npc));
                    world.player.character.unfreeze();
                } else {
                    window.text.update(ctx, delta);
                }
            } else {
                if npc.character.destination.is_some() {
                    npc.character.move_to_destination(delta);
                } else {
                    window.text.spawn();
                    world.player.input_frozen = true;
                    npc.character.destination = None;
    
                    let mut message_ran = false;
    
                    match &npc.interact {
                        NpcInteract::Message(pages) => {
                            window.text.set(pages.clone());
                            window.text.color(super::npc::npc_type(&npc.type_id).text_color);
                            message_ran = true;
                        },
                        NpcInteract::Script(_) => todo!(),
                        NpcInteract::Nothing => (),
                    }
                    
                    if !data_mut().world.get_map(&self.id).battled.contains(id) {
                        if let Some(trainer) = npc.trainer.as_ref() {

                            if trainer.battle_on_interact {

                                let npc_type = super::npc::npc_type(&npc.type_id);
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
                                        if let Some(playing_music) = audio::music::get_current_music() {
                                            if let Some(music) = audio::music::get_music_id(encounter_music).flatten() {
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
                        world.player.character.unfreeze();
                    }

                    if !message_ran {
                        window.text.despawn();
                        world.player.input_frozen = false;
                        let (id, npc) = self.npcs.active.take().unwrap();
                        self.npcs.list.insert(id, Some(npc));
                    } else {
                        window.text.process_messages(data());
                    }

                } 
            }
        }
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, door: &Option<manager::Door>, screen: &RenderCoords, border: bool, color: Color) {
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
                    textures.tiles.draw_tile(ctx, texture, tile, render_x, render_y, color);
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
                    textures.tiles.draw_tile(ctx, texture, tile, render_x, render_y, color);
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

pub fn despawn_script(script: &mut WorldScript) {
    data_mut().world.scripts.insert(script.identifier);
    script.despawn();
}

fn debug_input(ctx: &Context, map: &mut WorldMap) {
    if debug_pressed(ctx, DebugBind::F8) {
        for (index, npc) in map.npcs.list.iter().flat_map(|(id, npc)| npc.as_ref().map(|npc| (id, npc))) {
            info!("Npc {} (id: {}), is at {}, {}; looking {:?}", &npc.name, index, /*if npc.alive() {""} else {" (despawned)"},*/ &npc.character.position.coords.x, &npc.character.position.coords.y, &npc.character.position.direction);
        }
    }
}

fn find_battle(save: &mut PlayerSave, map: &Location, id: NpcId, npc: &mut Option<Npc>, active: &mut ActiveNpc, player: &mut PlayerCharacter) -> bool {
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