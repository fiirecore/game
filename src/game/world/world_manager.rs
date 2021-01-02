use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use std::collections::HashMap;

use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::util::map_traits::MapManager;
use serde_derive::Deserialize;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::io::data::player_data::PlayerData;

use crate::entity::entities::player::Player;
use crate::entity::entity::{Entity, Ticking};
use crate::util::traits::Loadable;
use crate::gui::gui::GuiComponent;

use crate::game::world::gui::map_window_manager::MapWindowManager;
use crate::game::world::gui::player_world_gui::PlayerWorldGui;

use crate::game::world::world_map::world_map_manager::WorldMapManager;

use crate::game::world::warp_map::warp_map_manager::WarpMapManager;

use crate::util::file_util::asset_as_pathbuf;

pub struct WorldManager {
    pub world_id: String,

    pub player_gui: PlayerWorldGui,

    pub window_manager: MapWindowManager,

    pub world_map_manager: WorldMapManager,
    pub warp_map_manager: WarpMapManager,

    pub player: Player,

    pub(crate) bottom_textures: HashMap<u16, Texture>,
    pub(crate) top_textures: HashMap<u16, Texture>,
    pub(crate) npc_textures: HashMap<u8, ThreeWayTexture>,

    pub palette_sizes: Vec<u16>,
    pub no_data_spawnpoint: [isize; 2],
}

pub(crate) static PALETTE_COUNT: u8 = 26;

impl WorldManager {

    pub fn new(player_data: &PlayerData) -> WorldManager {

        let id = player_data.location.world_id.as_str();

        let mut filename = String::from("worlds/");
        filename.push_str(id);
        filename.push_str("/");
        filename.push_str(id);
        filename.push_str(".toml");

        match std::fs::read_to_string(asset_as_pathbuf(filename)) {
            Ok(content) => {

                let toml: WorldConfig = toml::from_str(content.as_str()).unwrap();

                WorldManager {

                    world_id: player_data.location.world_id.clone(),

                    window_manager: MapWindowManager::new(),
                    player_gui: PlayerWorldGui::new(),

                    world_map_manager: WorldMapManager::new(),
                    warp_map_manager: WarpMapManager::new(),

                    player: Player::default(),

                    bottom_textures: HashMap::new(),
                    top_textures: HashMap::new(),
                    npc_textures: HashMap::new(),

                    palette_sizes: toml.palette_sizes.unwrap(),
                    no_data_spawnpoint: toml.no_data_spawnpoint.unwrap(),
                }

            }
            Err(error) => {
                panic!("Error opening toml file {}", error);
            }

        }

    }

    pub fn load(&mut self, player_data: &PlayerData) {
        self.bind_music();
        self.load_world(player_data);
    }

    pub fn load_world(&mut self, player_data: &PlayerData) {
        self.new_world_map(&player_data.location.world_id);

        self.load_player(player_data);
        self.load_maps(&player_data.location.world_id);
        self.load_npcs(&player_data.location.world_id);
        self.place_player(player_data);

        self.load_world_map();

        self.bottom_textures = HashMap::new();
        self.top_textures = HashMap::new();

        self.populate_textures();
    }

    pub fn on_start(&mut self, context: &mut GameContext) {
        music::set_volume(0.2);
        if self.world_map_manager.is_alive() {
            self.world_map_manager.on_start(context);
        } else {
            self.warp_map_manager.on_start(context);
        }
    }

    fn place_player(&mut self, player_data: &PlayerData) {
        if player_data.location.map_set_id.as_str().eq("world") {
            self.world_map_manager
                .get_current_world_mut()
                .current_piece_index = self
                .world_map_manager
                .get_current_world()
                .map_index_at_coordinates(self.player.coords.x, self.player.coords.y)
                .unwrap_or(0);
            self.world_map_manager.spawn();
        } else {
            self.warp_map_manager.spawn();
            self.warp_map_manager.load_set(
                &player_data.location.map_set_id,
                player_data.location.map_set_num,
            );
        }
    }

    pub fn update(&mut self, context: &mut GameContext) {
        self.player_movement(context);
        self.player.update(context);
        self.world_map_manager.update(context, &self.player);
        self.warp_map_manager.update(context, &self.player);
    }

    pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.player.focus_update();
        if self.world_map_manager.is_alive() {
            self.world_map_manager.render_below(ctx, g, tr, &self.bottom_textures, &self.npc_textures, &self.player);
            self.world_map_manager.render_above(ctx, g, tr, &self.top_textures, &self.player);
        } else {
            self.warp_map_manager.render_below(ctx, g, tr, &self.bottom_textures, &self.npc_textures, &self.player);
            self.warp_map_manager.render_above(ctx, g, tr, &self.top_textures, &self.player);
        }
        self.player_gui.render(ctx, g, tr);      
    }

    // Loading

    pub fn dispose(&mut self) {}
}

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub no_data_spawnpoint: Option<[isize; 2]>,
    pub palette_sizes: Option<Vec<u16>>,
}
