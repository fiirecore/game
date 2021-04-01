use firecore_world::map::set::WorldMapSet;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::map::warp::WarpDestination;
use macroquad::prelude::warn;

use crate::battle::data::BattleData;
use crate::world::NPCTypes;
use crate::world::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};
use crate::world::gui::text_window::TextWindow;
use firecore_world::character::player::PlayerCharacter;

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

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow, npc_types: &NPCTypes) {
        if let Some(map) = self.map_mut() {
            map.update(delta, player, battle_data, warp, text_window, npc_types);
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        if let Some(map) = self.map() {
            self.map().unwrap().render(tile_textures, npc_textures, npc_types, gui_textures, screen, border);
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

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow, npc_types: &NPCTypes) {
        if let Some(set) = self.set_mut() {
            set.update(delta, player, battle_data, warp, text_window, npc_types);
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        if let Some(set) = self.set() {
            set.render(tile_textures, npc_textures, npc_types, gui_textures, screen, border);
        }
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.input(delta, player);
        }
    }

}