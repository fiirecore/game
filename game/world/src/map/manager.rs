use game::{
    util::{
        Entity, 
        Completable, 
        Direction, 
        hash::HashMap,
    },
    pokedex::pokedex,
    data::{
        {get, get_mut},
        player::{PlayerSave, PlayerSaves},
    },
    input::{down, pressed, Control},
    macroquad::{
        prelude::{
            Image,
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

use firecore_world_lib::{
    serialized::SerializedWorld,
    map::{
        World,
        manager::WorldMapManager,
    },
    character::{Character, npc::npc_type::NPCType},
};

use crate::{
    GameWorld, TileTextures, NpcTextures, RenderCoords,
    player::PlayerTexture,
    warp_transition::WarpTransition,
    gui::{
        text_window::TextWindow,
        start_menu::StartMenu,
    },
    battle::random_wild_battle,
};

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

        info!("Loading maps...");
    
        info!("Loaded world file.");
    
        let images: Vec<(u8, Image)> = world.palettes.into_iter().map(|palette|
            (palette.id, Image::from_file_with_format(&palette.bottom, None))
        ).collect();
        
        let mut bottom_sheets = HashMap::new();
        let mut palette_sizes = HashMap::new();
        for (id, image) in images {
            palette_sizes.insert(id, (image.width >> 4) * (image.height >> 4));
            bottom_sheets.insert(id, image);
        }
    
        info!("Finished loading maps!");
    
        info!("Loading textures...");
        for tile_id in world.manager.chunk_map.tiles() {
            if !(self.tile_textures.textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                self.tile_textures.textures.insert(tile_id, get_texture(&bottom_sheets, &palette_sizes, tile_id));
            }
        }
        for tile_id in world.manager.map_set_manager.tiles() {
            if !(self.tile_textures.textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                self.tile_textures.textures.insert(tile_id, get_texture(&bottom_sheets, &palette_sizes, tile_id));
            }
        }
    
        info!("Loading NPC textures...");
    
        let mut npc_types = crate::npc::NPCTypes::with_capacity(world.npc_types.len());
        let mut trainer_sprites = TrainerSprites::new();
    
        for npc_type in world.npc_types {
            let texture = byte_texture(&npc_type.texture);
            if let Some(battle_sprite) = npc_type.battle_texture {
                trainer_sprites.insert(npc_type.config.identifier, byte_texture(&battle_sprite));
            }
            npc_types.insert(npc_type.config.identifier, NPCType {
                sprite: firecore_world_lib::character::sprite::SpriteIndexes::from_index(npc_type.config.sprite),
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
        
        if self.noclip_toggle && self.map_manager.player.position.local.offset.is_none() {
            self.noclip_toggle = false;
            self.map_manager.player.properties.noclip = !self.map_manager.player.properties.noclip;
            if self.map_manager.player.properties.noclip {
                self.map_manager.player.properties.speed = self.map_manager.player.properties.base_speed * 4.0;
            } else {
                self.map_manager.player.properties.speed = self.map_manager.player.properties.base_speed;
            }
            info!("No clip toggled!");
        }

        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window);
        } else {
            self.map_manager.map_set_manager.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window);
        }

        if self.warp_transition.is_alive() {
            self.warp_transition.update(delta);
            if self.warp_transition.switch() && !self.warp_transition.recognize_switch {
                self.warp_transition.recognize_switch = true;
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
                        self.map_manager.try_move(destination.position.direction.unwrap_or(self.map_manager.player.position.local.direction), delta);
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

        self.render_coords = RenderCoords::new(&self.map_manager.player);
        
    }

    pub fn render(&self) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.render(&self.tile_textures, &self.npc_textures, self.render_coords, true);
        } else {
            self.map_manager.map_set_manager.render(&self.tile_textures, &self.npc_textures, self.render_coords, true);
        }
        self.player_texture.render(&self.map_manager.player);
        self.text_window.render();
        self.start_menu.render(); 
        self.warp_transition.render();
    }

    pub fn save_data(&self, player_data: &mut PlayerSave) {
        if self.map_manager.chunk_active {
            player_data.location.map = None;
            player_data.location.index = self.map_manager.chunk_map.current.unwrap_or(firecore_game::data::player::default_index());
        } else {
            player_data.location.map = Some(self.map_manager.map_set_manager.current.unwrap_or(firecore_game::data::player::default_map()));
            player_data.location.index = self.map_manager.map_set_manager.set().map(|map| map.current).flatten().unwrap_or(firecore_game::data::player::default_index());
		}
		player_data.location.position = self.map_manager.player.position;
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
    
            if self.map_manager.player.position.local.offset.is_none() && !self.map_manager.player.is_frozen() {
                // self.map_manager.player.properties.moving = true;

                if down(Control::B) {
                    if !self.map_manager.player.properties.running {
                        self.map_manager.player.properties.running = true;
                        self.map_manager.player.properties.speed = 
                            ((self.map_manager.player.properties.base_speed as u8) << (
                                if self.map_manager.player.properties.noclip {
                                    2
                                } else {
                                    1
                                }
                            )) as f32;
                    }
                } else if self.map_manager.player.properties.running {
                    self.map_manager.player.properties.running = false;
                    self.map_manager.player.properties.reset_speed();
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
                        self.map_manager.player.position.local.direction = direction;
                        self.first_direction = direction;
                    } else {
                        self.player_move_accumulator = 0.0;
                        self.map_manager.player.properties.reset_speed();
                        self.map_manager.player.properties.moving = false;
                        self.map_manager.player.properties.running = false;
                    }
                }
            }
        }
        
    }

    fn stop_player(&mut self, battle_data: &mut Option<BattleData>) {
        self.map_manager.player.stop_move();

        // if self.map_manager.chunk_active {
        //     self.map_manager.ch
        // }

        if self.map_manager.chunk_active {
            if let Some(destination) = self.map_manager.chunk_map.check_warp(self.map_manager.player.position.local.coords) { // Warping does not trigger tile actions!
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.chunk_map.in_bounds(self.map_manager.player.position.local.coords) {
                self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
            }
        } else {
            if let Some(destination) = self.map_manager.map_set_manager.check_warp(self.map_manager.player.position.local.coords) {
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.map_set_manager.in_bounds(self.map_manager.player.position.local.coords) {
                self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
            }
        }        
    }

    pub fn load_player(&mut self, data: &PlayerSave) {
        let location = &data.location;
        self.map_manager.player.position = location.position;
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

            info!("Local Coordinates: {}", self.map_manager.player.position.local.coords);
            info!("Global Coordinates: ({}, {})", self.map_manager.player.position.get_x(), self.map_manager.player.position.get_y());

            if let Some(tile) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.tile(self.map_manager.player.position.local.coords)
            } else {
                self.map_manager.map_set_manager.tile(self.map_manager.player.position.local.coords)
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

                    let name = &map.name;
                    info!("Resetting battled trainers in this map! ({})", name);
                    saves.get_mut().world_status.get_or_create_map_data(name).battled.clear();

                }               
            }
        }

        if is_key_pressed(KeyCode::F6) {
            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                info!("Clearing used scripts in player data!");
                saves.get_mut().world_status.ran_scripts.clear();
            }
        }
        
    }
    
}

fn get_texture(sheets: &HashMap<u8, Image>, palette_sizes: &HashMap<u8, u16>, tile_id: u16) -> firecore_game::macroquad::prelude::Texture2D {

    use firecore_game::util::TILE_SIZE;
        
    let mut count: u16 = *palette_sizes.get(&0).unwrap();
    let mut index: u8 = 0;

    while tile_id >= count {
        index += 1;
        if index >= (palette_sizes.len() as u8) {
            firecore_game::macroquad::prelude::warn!("Tile ID {} exceeds palette texture count!", tile_id);
            break;
        }
        count += *palette_sizes.get(&index).unwrap();
    }

    match sheets.get(&index) {
        Some(sheet) => {
            let id = (tile_id - (count - *palette_sizes.get(&index).unwrap())) as usize;
            firecore_game::graphics::image_texture(
                &sheet.sub_image(
                    firecore_game::macroquad::prelude::Rect::new(
                        (id % (sheet.width() / TILE_SIZE as usize)) as f32 * TILE_SIZE, 
                        (id / (sheet.width() / TILE_SIZE as usize)) as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    )
                )
            )
        }
        None => {
            firecore_game::macroquad::prelude::debug!("Could not get texture for tile ID {}", &tile_id);
            firecore_game::graphics::debug_texture()
        }
    }
    
}