pub mod map;
pub mod warp;
pub mod npc;
pub mod pokemon;
pub mod gui {
    pub mod player_world_gui;
    pub mod map_window_manager;
}

pub mod player;

pub type NpcTextures = HashMap<String, ThreeWayTexture>;

use ahash::AHashMap as HashMap;
use crate::util::graphics::Texture;
use crate::util::TILE_SIZE;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::world::warp::WarpEntry;
use self::player::Player;

pub trait World {

    fn in_bounds(&self, x: isize, y: isize) -> bool;

    fn tile(&self, x: isize, y: isize) -> u16;

    fn walkable(&self, x: isize, y: isize) -> u8;

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry>;

    fn on_tile(&mut self, player: &mut Player);

    fn render(&self, textures: &HashMap<u16, Texture>, npc_textures: &NpcTextures, screen: RenderCoords, border: bool);

    fn input(&mut self, delta: f32, player: &mut Player);

}

#[derive(Default, Clone, Copy)]
pub struct RenderCoords {

    pub left: isize,
    pub right: isize,
    pub top: isize,
    pub bottom: isize,

    pub x_focus: f32,
    pub y_focus: f32,

    pub x_tile_offset: isize,
    pub y_tile_offset: isize,

}

static HALF_WIDTH: isize = (crate::BASE_WIDTH as isize + TILE_SIZE as isize) >> 1;
static HALF_HEIGHT: isize = (crate::BASE_HEIGHT as isize + TILE_SIZE as isize) >> 1;

static HALF_WIDTH_TILE: isize = HALF_WIDTH >> 4;
static HALF_HEIGHT_TILE: isize = (HALF_HEIGHT >> 4) + 2;

impl RenderCoords {

    pub fn new(player: &Player) -> Self {

        Self {

            left: player.position.get_x() - HALF_WIDTH_TILE,
            right: player.position.get_x() + HALF_WIDTH_TILE + 1,
            top: player.position.get_y() - HALF_HEIGHT_TILE,
            bottom: player.position.get_y() + HALF_HEIGHT_TILE,

            x_focus: (player.position.get_x() + 1 << 4) as f32 + player.position.local.x_offset - HALF_WIDTH as f32,
            y_focus: (player.position.get_y() + 1 << 4) as f32 + player.position.local.y_offset - HALF_HEIGHT as f32,

            ..Default::default()
        }

    }

    pub fn offset(&self, x: isize, y: isize) -> RenderCoords { // return offset x & y
        RenderCoords {
            x_tile_offset: x,
            y_tile_offset: y,
            ..*self
        }
    }

}