extern crate firecore_dependencies as deps;

mod timer;
pub mod positions;
mod bbox;

pub use {
    timer::Timer,
    positions::{
        direction::Direction,
        coordinate::*,
        pixel_offset::PixelOffset,
        position::Position,
        location::*,
        destination::Destination,
    },
    bbox::BoundingBox
};

pub const WIDTH: f32 = 240.0;
pub const HEIGHT: f32 = 160.0;
pub const TILE_SIZE: f32 = 16.0;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn alive(&self) -> bool;
	
}

pub trait Reset {

	fn reset(&mut self);

}

pub trait Completable: Reset {

    fn finished(&self) -> bool;

}

pub fn date() -> u64 {
    use std::time::SystemTime;
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|dur| dur.as_secs()).unwrap_or_default() % 1000
}