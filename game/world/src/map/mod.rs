use game::{
    deps::Random,
    util::{Entity, Completable, Direction, Timer},
    input::{pressed, Control},
    text::{Message, MessagePage, TextColor},
    storage::{
        get_mut,
        player::{
            PlayerSave,
            PlayerSaves,
        }
    },
    battle::BattleData,
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
    },
    script::world::{WorldScript, Condition, WorldActionKind, ScriptWarp},
};

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use super::gui::text_window::TextWindow;
use super::{GameWorld, WorldTextures, RenderCoords};
use crate::battle::{wild_battle, trainer_battle};

pub mod manager;
pub mod set;
pub mod chunk;

pub mod texture;
pub mod warp;

pub static NPC_RANDOM: Random = Random::new();
pub static mut NPC_TIMER: Timer = Timer::new(0.5);
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

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(tile_id) = self.tile(player.character.position.coords) {

            if WILD_ENCOUNTERS.load(Relaxed) {
                if let Some(wild) = &self.wild {
                    if wild.should_generate() {
                        if let Some(tiles) = wild.tiles.as_ref() {
                            for tile in tiles.iter() {
                                if tile_id.eq(tile) {
                                    wild_battle(battle_data, wild);
                                    break;
                                }
                            }
                        } else {
                            wild_battle(battle_data, wild);
                        }
                    }          
                }
            }            
    
            // look for player
            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                let save = saves.get_mut();
                for (index, npc) in self.npcs.iter_mut().filter(|(_, npc)| /*npc.is_alive() &&*/ npc.trainer.is_some()) {
                    find_battle(save, &self.id, index, npc, &mut self.state.npc, player);
                }            
            }


            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                let player_data = saves.get_mut();

                for script in self.scripts.iter_mut() {
    
                    if !script.is_alive() && script.in_location(&player.character.position.coords) {
                        let mut break_script = false;
                        for condition in &script.conditions {
                            match condition {
                                Condition::Scripts(scripts) => {
                                    for script_condition in scripts {
                                        if player_data.world.scripts.contains(&script_condition.identifier).ne(&script_condition.happened) {
                                            break_script = true;
                                        }  
                                    }                          
                                },
                                Condition::PlayerHasPokemon(is_true) => {
                                    if player_data.party.is_empty().eq(is_true) {
                                        break_script = true;
                                    }
                                }
                                _ => {
                                    break_script = true;
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
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {

        // Move NPCs

        for (index, npc) in self.npcs.iter_mut().filter(|(_, npc)| npc.character.destination.is_some() && npc.movement != MovementType::Still) {
            if self.state.npc.map(|active| active.ne(index)).unwrap_or(true) {
                npc.character.move_to_destination(delta);
            }            
        }

        if unsafe{NPC_TIMER.is_finished()} {
            unsafe{NPC_TIMER.soft_reset();}
            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                let save = saves.get_mut();
                for (index, npc) in self.npcs.iter_mut() {
                    if /*npc.is_alive() &&*/ npc.character.destination.is_none() && self.state.npc.map(|id| id.ne(index)).unwrap_or(true) {
                        if NPC_RANDOM.gen_float() < NPC_MOVE_CHANCE {
                            match npc.movement {
                                MovementType::Still => (),
                                MovementType::LookAround => {
                                    npc.character.position.direction = firecore_game::util::Direction::DIRECTIONS[NPC_RANDOM.gen_range(0, 4)];
                                    find_battle(save, &self.id, index, npc, &mut self.state.npc, player);
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
                                        if !find_battle(save, &self.id, index, npc, &mut self.state.npc, player) {
                                            if coords.y != player.character.position.coords.y {
                                                npc.character.go_to(coords);
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
                        WorldActionKind::PlayerGivePokemon(saved) => {
                            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                if saves.get().party.len() < 6 {
                                    saves.get_mut().party.push(saved.clone());
                                } else {
                                    warn!("Could not add pokemon #{} to player party because it is full", saved.id);
                                }
                            }
                            pop = true;
                        }
                        WorldActionKind::PlayerHealPokemon => {
                            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                for pokemon in saves.get_mut().party.iter_mut() {
                                    pokemon.current_hp = None;
                                    if let Some(moves) = pokemon.moves.as_mut() {
                                        for saved_move in moves {
                                            saved_move.pp = None;
                                        }
                                    }
                                }
                            }
                            pop = true;
                        }

                        WorldActionKind::PlayerGiveItem(item) => {
                            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                if !saves.get_mut().add_item(firecore_game::pokedex::item::ItemStack {
                                    id: *item,
                                    count: 1,
                                }) {
                                    warn!("Could not give item \"{}\"to player!", item);
                                }
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCAdd(id, npc) => {
                            if self.npcs.insert(*id, npc.clone()).is_some() {
                                warn!("Replaced NPC with id {}!", id);
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCRemove(id) => {
                            if self.npcs.remove(id).is_none() {
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
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if npc.interact_from(&player.character.position) {
                                    self.state.npc = Some(*id);
                                }
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCSay(id, message_set) => {
                            if let Some(npc) = self.npcs.get_mut(id) {
                                if let Some(npc_type) = super::npc::npc_type(&npc.npc_type) {
                                    if display_text(delta, text_window, &Message::new(npc_type.text_color, message_set.clone())) {
                                        pop = true;
                                    }
                                } else {
                                    pop = true;
                                }
                            } else {
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCBattle(id) => {
                            if let Some(npc) = self.npcs.get(id) {
                                trainer_battle(battle_data, &self.id, id, npc);
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
                            if display_text(delta, text_window, message) {
                                pop = true;
                            }
                        },

                        WorldActionKind::Conditional { message, end_message, unfreeze } => {

                            /*
                            * 0 = first message (default)
                            * 1 = end message
                            * 2 or 3 = yes/no option and cursor pos
                            */

                            if script.option == 0 {
                                if text_window.is_alive() {
                                    if text_window.is_finished() {
                                        script.option = 2;
                                    } else {
                                        text_window.update(delta);
                                    }
                                } else {
                                    text_window.spawn();
                                    text_window.set_text(message.clone());
                                }
                            } else if script.option == 1 {

                                if end_message.is_some() {

                                    if text_window.is_finished() {
                                        text_window.despawn();
                                        if *unfreeze {
                                            player.unfreeze();
                                        }
                                        script.option = 0;
                                        despawn_script(script);
                                    } else {
                                        text_window.update(delta);
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
                                        text_window.despawn();
                                        pop = true;
                                    } else if script.option == 3 {

                                        script.option = 1;
                                        if let Some(end_message) = end_message {
                                            text_window.reset_text();
                                            text_window.set_text(end_message.clone());
                                        }

                                    }
                                } else if pressed(Control::B) {

                                    script.option = 1;
                                    if let Some(end_message) = end_message {
                                        text_window.reset_text();
                                        text_window.set_text(end_message.clone());
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

        if let Some(npc) = 
        
        if let Some(active) = self.state.npc {
            if let Some(npc) = self.npcs.get_mut(&active) {
                Some(npc)
            } else {
                None
            }
        } else {
            None
        }
        
        {
            if text_window.is_alive() {
                if text_window.is_finished() {
                    {
                        trainer_battle(battle_data, &self.id, &self.state.npc.take().unwrap(), npc);
                    }
                    text_window.despawn();
                } else {
                    text_window.update(delta);
                }
            } else {
                if npc.character.destination.is_some() {
                    npc.character.move_to_destination(delta);
                } else {
                    text_window.spawn();
                    npc.character.destination = None;
    
                    let mut message_ran = false;
    
                    if let Some(messages) = npc.message.as_ref() {
                        text_window.set_text(
                            Message::new(
                                super::npc::npc_type(&npc.npc_type).map(|npc_type| npc_type.text_color).unwrap_or(TextColor::Black),
                            messages.clone()
                            ),
                        );
                        message_ran = true;
                    }
                    
                    if let Some(mut saves) = get_mut::<PlayerSaves>() {
                        if !saves.get_mut().world.get_map(&self.id).battled.contains(self.state.npc.as_ref().unwrap()) {
                            if let Some(trainer) = npc.trainer.as_ref() {
    
                                if trainer.battle_on_interact {

                                    if let Some(npc_type) = super::npc::npc_type(&npc.npc_type) {
                                        if let Some(trainer_type) = npc_type.trainer.as_ref() {

                                            // Spawn text window
                                                
                                            let messages = 
                                            Message::new(
                                                npc_type.text_color,
                                            trainer.encounter_message.iter().map(|message| {
                                                    MessagePage::new(
                                                        message.clone(),
                                                        None,
                                                    )
                                                }).collect()
                                            );
                                            text_window.set_text(
                                                messages,
                                            );
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
                        }
                    }
    
                    if !message_ran {
                        text_window.despawn();
                        self.state.npc = None;
                    } else {
                        if let Some(saves) = game::storage::get::<PlayerSaves>() {
                            text_window.process_messages(saves.get());
                        }
                    }
    
                    player.character.position.direction = npc.character.position.direction.inverse();
                    if player.character.is_frozen() {
                        player.unfreeze();
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
        for npc in self.npcs.values() {
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

    fn input(&mut self, _delta: f32, player: &mut PlayerCharacter) {

        if firecore_game::is_debug() {
            if is_key_pressed(KeyCode::F7) {
                player.character.freeze();
                player.character.unfreeze();
                player.character.noclip = true;
                info!("Unfroze player!");
            }
            if is_key_pressed(KeyCode::F8) {
                for (index, npc) in self.npcs.iter() {
                    info!("NPC {} (id: {}), is at {}, {}; looking {:?}", &npc.name, index, /*if npc.is_alive() {""} else {" (despawned)"},*/ &npc.character.position.coords.x, &npc.character.position.coords.y, &npc.character.position.direction);
                }
            }
            if is_key_pressed(KeyCode::F9) {
                let wild = !WILD_ENCOUNTERS.load(Relaxed);
                WILD_ENCOUNTERS.store(wild, Relaxed);
                info!("Wild Encounters: {}", wild);
            }
            if is_key_pressed(KeyCode::H) {
				if let Some(mut saves) = get_mut::<PlayerSaves>() {
					saves.get_mut().party.iter_mut().for_each(|pokemon| {
                        pokemon.current_hp = None;
                        pokemon.moves.as_mut().map(
                            | moves | 
                            moves.iter_mut().for_each(
                                | pmove | 
                                pmove.pp = None
                            )
                        );
                    });
				}
			}
        }

        if pressed(Control::A) {
            for (npc_index, npc) in self.npcs.iter_mut() {
                // if npc.is_alive() {
                    if npc.interact_from(&player.character.position) {
                        self.state.npc = Some(*npc_index);
                    }
                // }
            }
            for script in self.scripts.iter_mut() {
                if !script.is_alive() {
                    if script.in_location(&player.character.position.coords) {
                        let mut spawn = false;
                        for condition in script.conditions.iter() {
                            match condition {
                                Condition::Activate(direction) => {
                                    if player.character.position.direction.eq(direction) {
                                        spawn = true;
                                    }
                                }
                                _ => (),
                            }
                        }
                        if spawn {
                            script.spawn();
                        }
                    }
                }                
            }
        }
    }


}

pub fn despawn_script(script: &mut WorldScript) {
    if let Some(mut saves) = get_mut::<PlayerSaves>() {
        saves.get_mut().world.scripts.insert(script.identifier.clone());
    }
    script.despawn();
}

fn display_text(delta: f32, text_window: &mut TextWindow, message: &Message) -> bool {
    if text_window.is_alive() {
        if text_window.is_finished() {
            text_window.despawn();
            return true;
        } else {
            text_window.update(delta);
        }
    } else {
        text_window.spawn();
        text_window.set_text(message.clone());        
    }
    false
}

fn find_battle(save: &mut PlayerSave, map: &MapIdentifier, index: &NPCId, npc: &mut NPC, active: &mut Option<NPCId>, player: &mut PlayerCharacter) -> bool {
    if !save.world.has_battled(map, index) {
        if npc.find_character(&mut player.character) {
            *active = Some(*index);
            true
        } else {
            false
        }                                          
    } else {
        false
    }
}