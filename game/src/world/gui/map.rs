use crate::{
    deps::hash::HashMap,
    util::{Entity, Reset, Location},
    input::{pressed, Control},
    graphics::{draw_rectangle_lines, draw_rectangle, draw_text_left},
    tetra::{
        Context,
        math::Vec2,
        graphics::Color,
    },
};

pub type Locations = HashMap<Vec2<u8>, (String, Location)>;

#[derive(Default)]
pub struct WorldMapGui {
    alive: bool,

    selected: Vec2<u8>,

    locations: Locations,

    visited: Vec<Location>,
    fly: Option<Vec<Location>>,
}

pub enum MapIcon {
    City,
    Route,
}

impl WorldMapGui {

    pub const GUI_TILE_SIZE: f32 = 8.0;
    pub const GUI_TILE_SIZE_MASK: u8 = 3;

    pub fn add_locations(&mut self, locations: Locations) {
        self.locations = locations;
    }

    pub fn update(&mut self, ctx: &Context) {
        if pressed(ctx, Control::B) || pressed(ctx, Control::Select) {
            self.despawn();
        }
        if pressed(ctx, Control::Up) {
            self.selected.y = self.selected.y.saturating_sub(1);
        }
        if pressed(ctx, Control::Down) {
            self.selected.y = self.selected.y.wrapping_add(1);
        }
        if pressed(ctx, Control::Left) {
            self.selected.x = self.selected.x.saturating_sub(1);
        }
        if pressed(ctx, Control::Right) {
            self.selected.x = self.selected.x.wrapping_add(1);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let pos = self.selected << Self::GUI_TILE_SIZE_MASK;
        draw_rectangle_lines(ctx, pos.x as f32, pos.y as f32, Self::GUI_TILE_SIZE, Self::GUI_TILE_SIZE, 1.0, Color::WHITE);
        for pos in self.locations.keys() {
            let pos = *pos << Self::GUI_TILE_SIZE_MASK;
            draw_rectangle(ctx, pos.x as f32, pos.y as f32, Self::GUI_TILE_SIZE, Self::GUI_TILE_SIZE, Color::RED);
        }
        if let Some((name, _)) = self.locations.get(&self.selected) {
            draw_text_left(ctx, &1, name, &Color::WHITE, 5.0, util::HEIGHT - 20.0);
        }
    }

    pub fn despawn_get(&mut self) -> Option<Location> {
        let loc = self.locations.get(&self.selected).map(|(_, l)| *l);
        if loc.is_some() {
            self.despawn();
        }
        loc
    }

}

impl Entity for WorldMapGui {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Reset for WorldMapGui {
    fn reset(&mut self) {
        self.selected = Default::default();
    }
}