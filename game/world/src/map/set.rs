use firecore_world_lib::{
    map::{
        set::{
            WorldMapSet,
            manager::WorldMapSetManager,
        },
        warp::WarpDestination,
    },
    character::player::PlayerCharacter,
};

use firecore_game::battle::BattleData;

use firecore_game::macroquad::prelude::warn;

use crate::{GameWorld, TileTextures, NpcTextures, RenderCoords};
use crate::gui::text_window::TextWindow;

impl GameWorld for WorldMapSet {

    fn on_start(&mut self, music: bool) {
        if let Some(map) = self.map_mut() {
            map.on_start(music);
        } else {
            warn!("Could not get current map at {:?}!", self.current);
        }
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(map) = self.map_mut() {
            map.on_tile(battle_data, player);
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow) {
        if let Some(map) = self.map_mut() {
            map.update(delta, player, battle_data, warp, text_window);
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        if let Some(map) = self.map() {
            map.render(tile_textures, npc_textures, screen, border);
        }
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        if let Some(map) = self.map_mut() {
            map.input(delta, player);
        }
    }

}

impl GameWorld for WorldMapSetManager {

    fn on_start(&mut self, music: bool) {
        if let Some(set) = self.set_mut() {
            set.on_start(music);
        } else {
            warn!("Could not get current map set {:?}!", self.current);
        }
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.on_tile(battle_data, player);
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow) {
        if let Some(set) = self.set_mut() {
            set.update(delta, player, battle_data, warp, text_window);
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        if let Some(set) = self.set() {
            set.render(tile_textures, npc_textures, screen, border);
        }
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.input(delta, player);
        }
    }

}