pub extern crate firecore_world as worldlib;
pub extern crate firecore_pokedex_engine as pokengine;

// pub(extern) use firecore_world as world;

pub use pokengine::engine;
pub use pokengine::pokedex;

pub mod map;
mod gui;

pub fn color(message: worldlib::message::MessageColor) -> engine::graphics::Color {
    let a: [f32; 4] = message.into();
    a.into()
}