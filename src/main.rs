use args::*;

use engine::{
    tetra::{Result, ContextBuilder, time::Timestep},
    util::{WIDTH, HEIGHT},
};

use log::info;

use state::StateManager;

extern crate firecore_world as worldlib;
extern crate firecore_battle as battlelib;

pub mod world;
pub mod battle;

pub mod state;
pub mod args;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

fn main() -> Result {

    init::logger();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    let args = args();

    #[cfg(debug_assertions)]
    if !args.contains(&Args::NoSeed) {
        init::seed_random(std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).map(|dur| dur.as_secs()).unwrap_or_default() % 1000000)
    }

    #[cfg(feature = "discord")] 
    log::warn!("Discord support is broken!");
    // let mut client = {
    //     use discord_rich_presence::{new_client, DiscordIpc};
    //     use serde_json::json;
    //     let mut client = new_client("976382684683496880171344").unwrap();
    //     client.connect().unwrap();
    //     client.set_activity(json!({
    //         "state": "Test Game",
    //         // "details": "By DoNotDoughnut"
    //     })).unwrap();
    //     client
    // };
    
    // Loads configuration, sets up controls

    init::configuration()?;
    
    // Save data in local directory in debug builds
    #[cfg(debug_assertions)]
    storage::SAVE_IN_LOCAL_DIRECTORY.store(true, std::sync::atomic::Ordering::Relaxed);

    ContextBuilder::new(TITLE, (WIDTH * DEFAULT_SCALE) as _, (HEIGHT * DEFAULT_SCALE) as _)
    .resizable(true)
    .show_mouse(true)
    .timestep(Timestep::Variable)
    .build()?
    .run(|ctx| StateManager::new(ctx, args))?;

    // #[cfg(feature = "discord")]
    // discord_rich_presence::DiscordIpc::close(&mut client).unwrap();

    Ok(())

}

pub extern crate firecore_dependencies as deps;

pub extern crate firecore_engine as engine;
pub extern crate pokemon_firered_clone_storage as storage;
pub extern crate firecore_pokedex_game as pokedex;

pub extern crate simple_logger as logger;

pub mod battle_glue;
pub mod config;
pub mod game;
pub mod gui;
pub mod init;
pub mod text;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static QUIT: AtomicBool = AtomicBool::new(false);

pub fn quit() {
    QUIT.store(true, Relaxed)
}

#[inline(always)]
pub fn should_quit() -> bool {
    QUIT.load(Relaxed)
}

pub static DEBUG: AtomicBool = AtomicBool::new(cfg!(debug_assertions));

pub fn set_debug(debug: bool) {
    DEBUG.store(debug, Relaxed);
}

pub fn is_debug() -> bool {
    DEBUG.load(Relaxed)
}

#[cfg(feature = "world")]
pub fn keybind(direction: worldlib::positions::Direction) -> engine::input::Control {
    use worldlib::positions::Direction;
    use engine::input::Control;
    match direction {
        Direction::Up => Control::Up,
        Direction::Down => Control::Down,
        Direction::Left => Control::Left,
        Direction::Right => Control::Right,
    }
}
