mod panel;
mod button;
mod text;
mod bar;

pub use panel::*;
pub use button::*;
pub use text::*;
pub use bar::*;

pub mod party;
pub mod bag;

pub mod pokemon;

// pub struct StaticList<D, const SIZE: usize> {
//     pub options: [D; SIZE],
//     pub cursor: usize,
// }

// pub struct MultiStaticList<D: Array> {
//     pub options: 
//     pub cursor: usize,
// }