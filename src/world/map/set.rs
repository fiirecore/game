use firecore_world::map::set::WorldMapSet;
use firecore_world::map::set::manager::WorldMapSetManager;

use crate::world::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};
use crate::world::gui::text_window::TextWindow;
use firecore_world::character::player::PlayerCharacter;

impl GameWorld for WorldMapSet {

    fn on_start(&self, music: bool) {
        self.map().on_start(music);
    }

    fn on_tile(&mut self, player: &mut PlayerCharacter) {
        self.maps[self.current_map].on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, text_window: &mut TextWindow) {
        self.maps[self.current_map].update(delta, player, text_window);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        self.maps[self.current_map].render(tile_textures, npc_textures, gui_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.maps[self.current_map].input(delta, player)
    }

}

impl GameWorld for WorldMapSetManager {

    fn on_start(&self, music: bool) {
        self.map_set().on_start(music);
    }

    fn on_tile(&mut self, player: &mut PlayerCharacter) {
        self.map_set_mut().on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, text_window: &mut TextWindow) {
        self.map_set_mut().update(delta, player, text_window);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        self.map_set().render(tile_textures, npc_textures, gui_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.map_set_mut().input(delta, player)
    }

}