use firecore_world::map::set::WorldMapSet;
use firecore_world::map::set::manager::WorldMapSetManager;

use crate::world::GameWorld;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::TileTextures;
use crate::world::gui::map_window_manager::MapWindowManager;
use crate::world::player::Player;

impl GameWorld for WorldMapSet {

    fn on_tile(&mut self, player: &mut Player) {
        self.maps[self.current_map].on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager) {
        self.maps[self.current_map].update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.maps[self.current_map].render(tile_textures, npc_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.maps[self.current_map].input(delta, player)
    }

}

impl GameWorld for WorldMapSetManager {

    fn on_tile(&mut self, player: &mut Player) {
        self.map_set_mut().on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager) {
        self.map_set_mut().update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.map_set().render(tile_textures, npc_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.map_set_mut().input(delta, player)
    }

}