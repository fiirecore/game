mod timer;
mod positions;
mod bounding_box;
pub mod battle;

pub use {
    timer::Timer,
    positions::{
        direction::Direction,
        coordinate::Coordinate,
        pixel_offset::PixelOffset,
        position::Position,
        location::Location,
        destination::Destination,
    },
    bounding_box::BoundingBox
};

pub const WIDTH: f32 = 240.0;
pub const HEIGHT: f32 = 160.0;
pub const TILE_SIZE: f32 = 16.0;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

pub trait Reset {

	fn reset(&mut self);

}

pub trait Completable: Reset {

    fn is_finished(&self) -> bool;

}