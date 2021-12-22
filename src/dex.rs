use battlelib::pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex};

pub static mut POKEDEX: Option<BasicDex<Pokemon>> = None;
pub static mut MOVEDEX: Option<BasicDex<Move>> = None;
pub static mut ITEMDEX: Option<BasicDex<Item>> = None;

pub fn pokedex() -> &'static BasicDex<Pokemon> {
    unsafe { POKEDEX.as_ref().unwrap() }
}

pub fn movedex() -> &'static BasicDex<Move> {
    unsafe { MOVEDEX.as_ref().unwrap() }
}

pub fn itemdex() -> &'static BasicDex<Item> {
    unsafe { ITEMDEX.as_ref().unwrap() }
}
