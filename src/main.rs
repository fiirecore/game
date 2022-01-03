extern crate firecore_battle_engine as battlecli;
extern crate firecore_storage as storage;
extern crate firecore_world as worldlib;

pub(crate) use battlecli::battle;
pub(crate) use battle::pokedex;
pub(crate) use battlecli::pokedex::engine;
pub(crate) use battlecli::pokedex as pokengine;

use engine::{
    utils::{HEIGHT, WIDTH},
    ContextBuilder,
};
use state::StateManager;

mod battle_wrapper;
mod battle_glue;
mod command;
mod config;
mod dex;
mod load;
mod saves;
mod state;
mod world;

const TITLE: &str = "Pokemon FireRed";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const PUBLISHER: Option<&str> = Some("fiirecore");
const APPLICATION: &str = env!("CARGO_PKG_NAME");

const SCALE: f32 = 3.0;

fn main() {
    engine::run(
        ContextBuilder::new(TITLE, (WIDTH * SCALE) as _, (HEIGHT * SCALE) as _), // .resizable(true)
        // .show_mouse(true)
        load::OpenContext::load(),
            load::LoadContext::load,
        StateManager::new,
    );
}
