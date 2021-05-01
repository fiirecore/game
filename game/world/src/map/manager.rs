use firecore_world_lib::character::MoveType;
use game::{
    util::{
        Entity, 
        Completable, 
        Direction, 
    },
    pokedex::pokedex,
    storage::{
        {get, get_mut},
        player::{PlayerSave, PlayerSaves},
    },
    input::{down, pressed, Control},
    macroquad::{
        prelude::{
            KeyCode,
            info,
            is_key_pressed,
        }
    },
    textures::{TrainerSprites, TRAINER_SPRITES},
    graphics::byte_texture,
    battle::BattleData,
    gui::{
        party::PartyGui,
        bag::BagGui,
    },
    state::GameStateAction,
};

use world::{
    serialized::SerializedWorld,
    map::{
        World,
        manager::WorldMapManager,
    },
    character::npc::npc_type::NPCType,
};

use crate::{
    GameWorld, TileTextures, NpcTextures, RenderCoords,
    player::PlayerTexture,
    gui::{
        text_window::TextWindow,
        start_menu::StartMenu,
    },
    battle::random_wild_battle,
};

use super::warp::WarpTransition;

const PLAYER_MOVE_TIME: f32 = 0.12;

pub struct WorldManager {

    pub map_manager: WorldMapManager,

    tile_textures: TileTextures,
    npc_textures: NpcTextures,
    pub player_texture: PlayerTexture,

    warp_transition: WarpTransition,

    start_menu: StartMenu,
    text_window: TextWindow,

    // Other

    pub render_coords: RenderCoords,
    noclip_toggle: bool,

    first_direction: Direction,
    player_move_accumulator: f32,

}

impl WorldManager {

    pub fn new() -> Self {        
        Self {

            map_manager: WorldMapManager::default(),

            tile_textures: TileTextures::new(),
            npc_textures: NpcTextures::new(),
            player_texture: PlayerTexture::default(),

            warp_transition: WarpTransition::new(),
            start_menu: StartMenu::new(),
            text_window: TextWindow::default(),
            first_direction: Direction::default(),
            render_coords: RenderCoords::default(),
            noclip_toggle: false,
            player_move_accumulator: 0.0,
        }
    }

    pub  fn load(&mut self, world: SerializedWorld) {
    
        self.tile_textures.setup(world.textures);
    
        let mut npc_types = crate::npc::NPCTypes::with_capacity(world.npc_types.len());
        let mut trainer_sprites = TrainerSprites::new();
    
        for npc_type in world.npc_types {
            let texture = byte_texture(&npc_type.texture);
            if let Some(battle_sprite) = npc_type.battle_texture {
                trainer_sprites.insert(npc_type.config.identifier, byte_texture(&battle_sprite));
            }
            npc_types.insert(npc_type.config.identifier, NPCType {
                sprite: firecore_world_lib::character::sprite::SpriteIndexes::from_index(npc_type.config.sprite),
                text_color: npc_type.config.text_color,
                trainer: npc_type.config.trainer,
            });
            self.npc_textures.insert(npc_type.config.identifier, texture);
        }
    
        unsafe {crate::npc::NPC_TYPES = Some(npc_types); }
    
        unsafe { TRAINER_SPRITES = Some(trainer_sprites); }

        crate::gui::load();
        
        info!("Finished loading textures!");

        self.map_manager = world.manager;
    
    }

    pub fn load_with_data(&mut self) {
        if let Some(saves) = get::<PlayerSaves>() {
            let save = saves.get();
            self.load_player(save);
        }
    }

    pub fn on_start(&mut self, battle_data: &mut Option<BattleData>) {
        self.map_start(true);
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
        } else {
            self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
        }
    }

    pub fn map_start(&mut self, music: bool) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_start(music);
        } else {
            self.map_manager.map_set_manager.on_start(music);
        }
    }

    pub fn update(&mut self, delta: f32, battle_data: &mut Option<BattleData>) {
        self.tile_textures.update(delta);
        
        if self.noclip_toggle && self.map_manager.player.character.position.offset.is_zero() {
            self.noclip_toggle = false;
            self.map_manager.player.character.noclip = !self.map_manager.player.character.noclip;
            // if self.map_manager.player.character.noclip {
            //     self.map_manager.player.character.speed = self.map_manager.player.character.base_speed * 4.0;
            // } else {
            //     self.map_manager.player.character.speed = self.map_manager.player.character.base_speed;
            // }
            info!("No clip toggled!");
        }

        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window);
        } else {
            self.map_manager.map_set_manager.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window);
        }

        if self.warp_transition.is_alive() {
            self.warp_transition.update(delta);
            if self.warp_transition.switch() {
                if let Some((destination, music)) = self.map_manager.warp.clone() {
                    self.player_texture.draw = !destination.transition.move_on_exit;
                    self.map_manager.warp(destination);
                    self.map_start(music);
                    if self.map_manager.chunk_active {
                        self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
                    } else {
                        self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
                    }
                }
            }
            if self.warp_transition.is_finished() {
                self.player_texture.draw = true;
                self.warp_transition.despawn();
                self.map_manager.player.unfreeze();
                if let Some((destination, _)) = self.map_manager.warp.take() {
                    if destination.transition.move_on_exit {
                        self.map_manager.try_move(destination.position.direction.unwrap_or(self.map_manager.player.character.position.direction), delta);
                    }
                }
                
            }
        } else {
            if self.map_manager.warp.is_some() {
                self.warp_transition.spawn();
                self.map_manager.player.freeze_input();
            }
        }

        if !self.map_manager.player.is_frozen() {
            if self.map_manager.player.do_move(delta) {
                self.stop_player(battle_data);
            }
        }

        let offset = if self.map_manager.chunk_active {
            self.map_manager.chunk_map.chunk().map(|chunk| chunk.coords)
        } else {
            None
        }.unwrap_or(firecore_game::util::Coordinate { x: 0, y: 0 });

        self.render_coords = RenderCoords::new(offset, &self.map_manager.player.character);
        
    }

    pub fn render(&self) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.render(&self.tile_textures, &self.npc_textures, self.render_coords, true);
        } else {
            self.map_manager.map_set_manager.render(&self.tile_textures, &self.npc_textures, self.render_coords, true);
        }
        self.player_texture.render(&self.map_manager.player.character);
        self.text_window.render();
        self.start_menu.render(); 
        self.warp_transition.render();
    }

    pub fn save_data(&self, player_data: &mut PlayerSave) {
        use firecore_game::storage::player;
        if self.map_manager.chunk_active {
            player_data.location.map = None;
            player_data.location.index = self.map_manager.chunk_map.current.unwrap_or(player::default_index());
        } else {
            player_data.location.map = Some(self.map_manager.map_set_manager.current.unwrap_or(player::default_map()));
            player_data.location.index = self.map_manager.map_set_manager.set().map(|map| map.current).flatten().unwrap_or(player::default_index());
		}
		player_data.location.position = self.map_manager.player.character.position;
    }

    pub fn input(&mut self, delta: f32, battle_data: &mut Option<BattleData>, party_gui: &mut PartyGui, bag_gui: &mut BagGui, action: &mut Option<GameStateAction>) {

        if firecore_game::is_debug() {
            self.debug_input(battle_data)
        }

        if pressed(Control::Start) {
            self.start_menu.toggle();
        }

        if self.text_window.is_alive() {
            self.text_window.input();
        } else if self.start_menu.is_alive() {
            self.start_menu.input(action, party_gui, bag_gui);
        } else {

            if self.map_manager.chunk_active {
                self.map_manager.chunk_map.input(delta, &mut self.map_manager.player);
            } else {
                self.map_manager.map_set_manager.input(delta, &mut self.map_manager.player);
            }
    
            if !(!self.map_manager.player.character.position.offset.is_zero() || self.map_manager.player.is_frozen() ) {

                if down(Control::B) {
                    if self.map_manager.player.character.move_type == MoveType::Walking {
                        self.map_manager.player.character.move_type = MoveType::Running;
                    }
                } else if self.map_manager.player.character.move_type == MoveType::Running {
                    self.map_manager.player.character.move_type = MoveType::Walking;
                }

                if down(firecore_game::keybind(self.first_direction)) {
                    if self.player_move_accumulator > PLAYER_MOVE_TIME {
                        if self.map_manager.try_move(self.first_direction, delta) {
                            self.map_start(true);
                        }
                    } else {
                        self.player_move_accumulator += delta;
                    }
                } else {
                    let mut movdir: Option<Direction> = None;
                    for direction in &firecore_game::util::Direction::DIRECTIONS {
                        if down(firecore_game::keybind(*direction)) {
                            movdir = if let Some(dir) = movdir {
                                if dir.inverse().eq(direction) {
                                    None
                                } else {
                                    Some(*direction)
                                }
                            } else {
                                Some(*direction)
                            };
                        }                        
                    }
                    if let Some(direction) = movdir {
                        self.map_manager.player.character.position.direction = direction;
                        self.first_direction = direction;
                    } else {
                        self.player_move_accumulator = 0.0;
                        self.map_manager.player.character.moving = false;
                    }
                }
            }
        }
        
    }

    fn stop_player(&mut self, battle_data: &mut Option<BattleData>) {
        self.map_manager.player.character.stop_move();

        // if self.map_manager.chunk_active {
        //     self.map_manager.ch
        // }

        if self.map_manager.chunk_active {
            if let Some(destination) = self.map_manager.chunk_map.check_warp(self.map_manager.player.character.position.coords) { // Warping does not trigger tile actions!
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.chunk_map.in_bounds(self.map_manager.player.character.position.coords) {
                self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
            }
        } else {
            if let Some(destination) = self.map_manager.map_set_manager.check_warp(self.map_manager.player.character.position.coords) {
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.map_set_manager.in_bounds(self.map_manager.player.character.position.coords) {
                self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
            }
        }        
    }

    pub fn load_player(&mut self, data: &PlayerSave) {
        let location = &data.location;
        self.map_manager.player.character.position = location.position;
        self.player_texture.load();

        if let Some(map) = location.map {
            self.map_manager.chunk_active = false;
            self.map_manager.update_map_set(map, location.index);
        } else {
            self.map_manager.chunk_active = true;
            self.map_manager.update_chunk(location.index);
        }     
    }

    fn debug_input(&mut self, battle_data: &mut Option<BattleData>) {

        if is_key_pressed(KeyCode::F1) {
            random_wild_battle(battle_data);
        }

        if is_key_pressed(KeyCode::F2) {
            self.noclip_toggle = true;
        }

        if is_key_pressed(KeyCode::F3) {

            info!("Local Coordinates: {}", self.map_manager.player.character.position.coords);
            // info!("Global Coordinates: ({}, {})", self.map_manager.player.position.get_x(), self.map_manager.player.position.get_y());

            if let Some(tile) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.tile(self.map_manager.player.character.position.coords)
            } else {
                self.map_manager.map_set_manager.tile(self.map_manager.player.character.position.coords)
            } {
                info!("Current Tile ID: {}", tile);
            } else {
                info!("Currently out of bounds");
            }
            
        }

        if is_key_pressed(KeyCode::F4) {
            if let Some(saves) = get::<PlayerSaves>() {
                for (slot, instance) in saves.get().party.iter().enumerate() {
                    if let Some(pokemon) = pokedex().get(&instance.id) {
                        info!("Party Slot {}: Lv{} {}", slot, instance.data.level, pokemon.data.name);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(mut saves) = get_mut::<PlayerSaves>() {

                if let Some(map) = if self.map_manager.chunk_active {
                    self.map_manager.chunk_map.chunk().map(|chunk| &chunk.map)
                } else {
                    self.map_manager.map_set_manager.set().map(|map| map.map()).flatten()
                } {

                    info!("Resetting battled trainers in this map! ({})", map.name);
                    saves.get_mut().world.get_map(&map.id).battled.clear();

                }               
            }
        }

        if is_key_pressed(KeyCode::F6) {
            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                info!("Clearing used scripts in player data!");
                saves.get_mut().world.scripts.clear();
            }
        }
        
    }
    
}