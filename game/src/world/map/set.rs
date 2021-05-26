use crate::{
    battle_glue::BattleEntryRef,
    tetra::Context,
    log::warn,
};

use worldlib::{
    map::{
        set::{
            WorldMapSet,
            manager::WorldMapSetManager,
        },
        warp::WarpDestination,
    },
    character::player::PlayerCharacter,
};

use crate::world::{
    GameWorld,
    WorldTextures,
    RenderCoords,
    gui::TextWindow,
};

impl GameWorld for WorldMapSet {

    fn on_start(&mut self, ctx: &mut Context, music: bool) {
        if let Some(map) = self.map_mut() {
            map.on_start(ctx, music);
        } else {
            warn!("Could not get current map at {:?}!", self.current);
        }
    }

    fn on_tile(&mut self, ctx: &mut Context, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(map) = self.map_mut() {
            map.on_tile(ctx, battle, player);
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(map) = self.map_mut() {
            map.update(ctx, delta, player, battle, warp, text_window);
        }
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, screen: &RenderCoords, border: bool) {
        if let Some(map) = self.map() {
            map.draw(ctx, textures, screen, if border { map.border[0] != 0x8 } else { false });
        }
    }

}

impl GameWorld for WorldMapSetManager {

    fn on_start(&mut self, ctx: &mut Context, music: bool) {
        if let Some(set) = self.set_mut() {
            set.on_start(ctx, music);
        } else {
            warn!("Could not get current map set {:?}!", self.current);
        }
    }

    fn on_tile(&mut self, ctx: &mut Context, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(set) = self.set_mut() {
            set.on_tile(ctx, battle, player);
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(set) = self.set_mut() {
            set.update(ctx, delta, player, battle, warp, text_window);
        }
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, screen: &RenderCoords, border: bool) {
        if let Some(set) = self.set() {
            set.draw(ctx, textures, screen, border);
        }
    }

}