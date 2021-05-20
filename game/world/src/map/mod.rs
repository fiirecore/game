use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use game::{
    deps::Random,
    util::{Entity, Completable, Direction, Timer},
    pokedex::{
        Identifiable,
        item::{
            Item,
            ItemStack,
        },
    },
    input::{pressed, Control},
    text::MessagePage,
    storage::{
        data, data_mut,
        player::PlayerSave,
    },
    battle::BattleEntryRef,
    play_music_named, play_music, 
    graphics::{draw, draw_cursor},
    macroquad::prelude::{KeyCode, info, warn, is_key_pressed},
};

use world::{
    TileId,
    character::{
        movement::MovementType,
        npc::{NPC, NPCId},
        player::PlayerCharacter,
    },
    map::{
        MapIdentifier,
        World,
        WorldMap,
        warp::WarpDestination,
        manager::can_move,
        ActiveNPC,
    },
    script::world::{WorldScript, Condition, WorldActionKind, ScriptWarp},
};

use crate::{
    GameWorld,
    WorldTextures,
    RenderCoords,
    gui::TextWindow,
    battle::{wild_battle, trainer_battle}
};

pub mod manager;
pub mod set;
pub mod chunk;

pub mod texture;
pub mod warp;

pub static NPC_RANDOM: Random = Random::new();
pub static mut NPC_TIMER: Timer = Timer::new(true, 0.5);
pub static WILD_ENCOUNTERS: AtomicBool = AtomicBool::new(true);

const NPC_MOVE_CHANCE: f32 = 1.0 / 12.0;

impl GameWorld for WorldMap {

    fn on_start(&mut self, music: bool) {

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
            if firecore_game::audio::get_current_music().map(|current| current != self.music).unwrap_or(true) {
                play_music(self.music);
            }
        }

    }

    fn on_tile(&mut self, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(tile_id) = self.tile(player.character.position.coords) {

            if WILD_ENCOUNTERS.load(Relaxed) {
                if let Some(wild) = &self.wild {
                    if wild.should_generate() {
                        if let Some(tiles) = wild.tiles.as_ref() {
                            for tile in tiles.iter() {
                                if tile_id.eq(tile) {
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
                find_battle(save, &self.id, *index, npc, &mut self.npcs.active, player);
            }

            for script in self.scripts.iter_mut() {
                if !script.is_alive() && script.in_location(&player.character.position.coords) {
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
                                if player.character.position.direction.ne(direction) {
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

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, window: &mut TextWindow) {

        if firecore_game::is_debug() {
            debug_input(self);
        }

        if pressed(Control::A) && self.npcs.active.is_none() {
            for (id, npc_opt) in self.npcs.list.iter_mut() {
                if let Some(npc) = npc_opt {
                    if npc.message.is_some() || npc.trainer.is_some() {
                        if npc.interact_from(&player.character.position) {
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

        if unsafe{NPC_TIMER.is_finished()} {
            unsafe{NPC_TIMER.soft_reset();}
            let save = data_mut();
            for (index, npc_opt) in self.npcs.list.iter_mut() {
                if let Some(npc) = npc_opt {
                    if npc.character.destination.is_none() {
                        if NPC_RANDOM.gen_float() < NPC_MOVE_CHANCE {
                            match npc.movement {
                                MovementType::Still => (),
                                MovementType::LookAround => {
                                    npc.character.position.direction = firecore_game::util::Direction::DIRECTIONS[NPC_RANDOM.gen_range(0, 4)];
                                    find_battle(save, &self.id, *index, npc_opt, &mut self.npcs.active, player);
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
                                        if !find_battle(save, &self.id, *index, npc_opt, &mut self.npcs.active, player) {
                                            if coords.y != player.character.position.coords.y {
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

        for script in self.scripts.iter_mut().filter(|script| script.is_alive()) {
            let mut pop = false;
            match script.actions.front() {
                Some(action) => {
                    match action {
                        WorldActionKind::PlayMusic(music) => {
                            play_music_named(&music);
                            pop = true;
                        },
                        WorldActionKind::PlayMapMusic => {
                            play_music(self.music);
                            pop = true;
                        },
                        WorldActionKind::PlaySound(sound) => {
                            if let Err(err) = firecore_game::audio::play_sound(sound) {
                                warn!("Could not play sound {:?} for script {} with error {}", sound, script.identifier, err);
                            }
                            pop = true;
                        }
                        WorldActionKind::PlayerFreezeInput => {
                            player.input_frozen = true;
                            player.character.stop_move();
                            pop = true;
                        },
                        WorldActionKind::PlayerUnfreezeInput => {
                            player.input_frozen = false;
                            pop = true;
                        }
                        WorldActionKind::PlayerUnfreeze => {
                            player.character.frozen = false;
                            pop = true;
                        }
                        WorldActionKind::PlayerLook(direction) => {
                            player.character.position.direction = *direction;
                            pop = true;
                        }
                        WorldActionKind::PlayerMove(destination) => {
                            if player.character.destination.is_some() {
                                if player.character.move_to_destination(delta) {
                                    pop = true;
                                }
                            } else {
                                player.character.move_to(*destination);
                            }
                        }
                        WorldActionKind::PlayerGivePokemon(instance) => {
                            if let Err(err) = data_mut().party.try_push(instance.clone()) {
                                warn!("Could not add {} to player's party with error {}", instance.pokemon.value().data.name, err);
                            }
                            pop = true;
                        }
                        WorldActionKind::PlayerHealPokemon => {
                            for pokemon in data_mut().party.iter_mut() {
                                pokemon.current_hp = pokemon.base.hp;
                                for pmove in pokemon.moves.as_mut() {
                                    pmove.pp = pmove.move_ref.value().pp;
                                }
                            }
                            pop = true;
                        }

                        WorldActionKind::PlayerGiveItem(item) => {
                            data_mut().add_item(ItemStack {
                                item: Item::get(item),
                                count: 1,
                            });
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
                                if player.character.destination.is_some() {
                                    if player.character.move_to_destination(delta) {
                                        pop = true;
                                    }
                                } else {
                                    if player.character.position.coords.ne(&pos.coords) {
                                        player.character.destination = npc.character.destination.clone();
                                        if let Some(destination) = player.character.destination.as_mut() {
                                            destination.queued_movements.pop_back();
                                            destination.queued_movements.push_front(player.character.position.coords.towards(npc.character.position.coords))
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
                                    npc.character.go_next_to(player.character.position.coords)
                                }
                            } else {
                                warn!("NPC script tried to move to player with an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCInteract(id) => {
                            if let Some(npc) = self.npcs.list.get_mut(id) {
                                if npc.as_mut().map(|npc| npc.interact_from(&player.character.position)).unwrap_or_default() {
                                    self.npcs.active = Some((*id, npc.take().unwrap()));
                                }
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCSay(id, pages) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if window.text.is_alive() {
                                    if !window.text.is_finished() {
                                        window.text.update(delta);
                                    } else {
                                        window.text.despawn();
                                        pop = true;
                                    }
                                } else {
                                    window.text.clear();
                                    window.text.set(pages.clone());
                                    window.text.color(crate::npc::npc_type(&npc.npc_type).text_color);
                                    window.text.process_messages(data()); 
                                    window.text.spawn();   
                                }
                            } else {
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCBattle(id) => {
                            if let Some(npc) = self.npcs.get(id) {
                                trainer_battle(battle, npc, &self.id, id);
                            }
                            pop = true;
                        }

                        WorldActionKind::Wait(time) => {
                            if script.timer.is_alive() {
                                script.timer.update(delta);
                                if script.timer.is_finished() {
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
                            if window.text.is_alive() {
                                if !window.text.is_finished() {
                                    window.text.update(delta);
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
                                if window.text.is_alive() {
                                    if window.text.is_finished() {
                                        script.option = 2;
                                    } else {
                                        window.text.update(delta);
                                    }
                                } else {
                                    window.text.clear();
                                    window.text.spawn();
                                    window.text.set(message.pages.clone());
                                    window.text.color(message.color);
                                }
                            } else if script.option == 1 {

                                if end_message.is_some() {

                                    if window.text.is_finished() {
                                        window.text.despawn();
                                        if *unfreeze {
                                            player.unfreeze();
                                        }
                                        script.option = 0;
                                        despawn_script(script);
                                    } else {
                                        window.text.update(delta);
                                    }

                                } else {
                                    if *unfreeze {
                                        player.unfreeze();
                                    }
                                    script.option = 0;
                                    despawn_script(script);
                                }
                            } else {
                                if pressed(Control::A) {
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
                                } else if pressed(Control::B) {

                                    script.option = 1;
                                    if let Some(end_message) = end_message {
                                        window.text.clear();
                                        window.text.set(end_message.pages.clone());
                                        window.text.color(end_message.color);
                                    }

                                }
                                if pressed(Control::Up) && script.option == 3 {
                                    script.option = 2;
                                }
                                if pressed(Control::Down) && script.option == 2 {
                                    script.option = 3;
                                }
                            }
                        }

                        WorldActionKind::Warp(warp_type) => {
                            *warp = Some(match warp_type {
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
                            player.character.destination = None; // fix so this is not necessary
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

        #[deprecated(note = "rewrite active NPC code")]
        if let Some((id, npc)) = self.npcs.active.as_mut() {
            if window.text.is_alive() {
                if window.text.is_finished() {
                    trainer_battle(battle, npc, &self.id, id);
                    window.text.despawn();
                    let (id, npc) = self.npcs.active.take().unwrap();
                    self.npcs.list.insert(id, Some(npc));
                    player.unfreeze();
                } else {
                    window.text.update(delta);
                }
            } else {
                if npc.character.destination.is_some() {
                    npc.character.move_to_destination(delta);
                } else {
                    window.text.spawn();
                    player.freeze_input();
                    npc.character.destination = None;
    
                    let mut message_ran = false;
    
                    if let Some(pages) = npc.message.as_ref() {
                        window.text.set(pages.clone());
                        window.text.color(super::npc::npc_type(&npc.npc_type).text_color);
                        message_ran = true;
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
                                        if let Some(playing_music) = firecore_game::audio::get_current_music() {
                                            if let Some(music) = firecore_game::audio::get_music_id(encounter_music).flatten() {
                                                if playing_music != music {
                                                    firecore_game::play_music(music)
                                                }
                                            }
                                        } else {
                                            firecore_game::play_music_named(encounter_music)
                                        }
                                    }
                                }
                            }
                        }   
                    }
    
                    player.character.position.direction = npc.character.position.direction.inverse();
                    if player.character.is_frozen() {
                        player.unfreeze();
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

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool) {
        let primary = *textures.tiles.palettes.get(&self.palettes[0]).expect("Could not get primary palette for map!");
        let length = primary.height() as TileId;
        let secondary = *textures.tiles.palettes.get(&self.palettes[1]).expect("Could not get secondary palette for map!");

        for yy in screen.top..screen.bottom {
            let y = yy - screen.offset.y;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            let row = (y as usize).saturating_mul(self.width);
            
            for xx in screen.left..screen.right {
                let x = xx - screen.offset.x;
                let render_x = (xx << 4) as f32 - screen.focus.x;

                if !(x < 0 || y < 0 || y >= self.height as _ || x >= self.width as _) {
                    let tile = self.tiles[x as usize + row as usize];
                    let (texture, tile) = if length > tile { (primary, tile) } else { (secondary, tile - length) };
                    textures.tiles.render_tile(texture, tile, render_x, render_y);
                } else if border {
                    let tile = self.border[if x % 2 == 0 { //  x % 2 + if y % 2 == 0 { 0 } else { 2 }
                        if y % 2 == 0 { 0 } else { 2 }
                    } else {
                        if y % 2 == 0 { 1 } else { 3 }
                    }];
                    let (texture, tile) = if length > tile { (primary, tile) } else { (secondary, tile - length) };
                    textures.tiles.render_tile(texture, tile, render_x, render_y);
                }
            }
        }
        for npc in self.npcs.list.values().flatten() {
            textures.npcs.render(npc, &screen);
        }
        if let Some((_, npc)) = self.npcs.active.as_ref() {
            textures.npcs.render(npc, &screen);
        }
        for script in self.scripts.iter() {
            if script.is_alive() {
                if let Some(action) = script.actions.front() {
                    match action {
                        WorldActionKind::Conditional{ .. } => {
                                if script.option > 1 {
                                    draw(textures.gui.get(&texture::gui::GuiTexture::Condition), 162.0, 66.0);
                                    draw_cursor(170.0, 77.0 + (script.option - 2) as f32 * 16.0);
                                }
                        }
                        _ => (),
                    }                    
                }
            }            
        }
    }
}

fn debug_input(map: &mut WorldMap) {
    if is_key_pressed(KeyCode::F8) {
        for (index, npc) in map.npcs.list.iter().flat_map(|(id, npc)| npc.as_ref().map(|npc| (id, npc))) {
            info!("NPC {} (id: {}), is at {}, {}; looking {:?}", &npc.name, index, /*if npc.is_alive() {""} else {" (despawned)"},*/ &npc.character.position.coords.x, &npc.character.position.coords.y, &npc.character.position.direction);
        }
    }
}

pub fn despawn_script(script: &mut WorldScript) {
    data_mut().world.scripts.insert(script.identifier);
    script.despawn();
}

fn find_battle(save: &mut PlayerSave, map: &MapIdentifier, id: NPCId, npc: &mut Option<NPC>, active: &mut ActiveNPC, player: &mut PlayerCharacter) -> bool {
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