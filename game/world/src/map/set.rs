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

use firecore_game::battle::BattleEntryRef;

use firecore_game::macroquad::prelude::warn;

use crate::{GameWorld, WorldTextures, RenderCoords};
use crate::gui::text_window::TextWindow;

impl GameWorld for WorldMapSet {

    fn on_start(&mut self, music: bool) {
        if let Some(map) = self.map_mut() {
            map.on_start(music);
        } else {
            warn!("Could not get current map at {:?}!", self.current);
        }
    }

    fn on_tile(&mut self, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(map) = self.map_mut() {
            map.on_tile(battle, player);
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(map) = self.map_mut() {
            map.update(delta, player, battle, warp, text_window);
        }
    }

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool) {
        if let Some(map) = self.map() {
            map.render(textures, screen, if border { map.border[0] != 0x8 } else { false });
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

    fn on_tile(&mut self, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.on_tile(battle, player);
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(set) = self.set_mut() {
            set.update(delta, player, battle, warp, text_window);
        }
    }

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool) {
        if let Some(set) = self.set() {
            set.render(textures, screen, border);
        }
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.input(delta, player);
        }
    }

}