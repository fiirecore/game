pub extern crate firecore_dependencies as deps;
extern crate firecore_storage;
pub extern crate firecore_engine as engine;
pub extern crate firecore_pokedex_client as pokedex;

pub mod world;
pub mod battle;
pub mod game;
pub mod state;
pub mod args;

use game::{init, storage};

use engine::{
    tetra::{Result, ContextBuilder, time::Timestep},
    util::{WIDTH, HEIGHT},
};

use log::info;

use state::StateManager;

extern crate firecore_world as worldlib;
extern crate firecore_battle as battlelib;

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
    storage::should_save_locally(true);

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

#[derive(PartialEq)]
pub enum Args {

    DisableAudio,
    Debug,
    #[cfg(debug_assertions)]
    NoSeed,

}

pub fn args() -> Vec<Args> {
    let mut list = Vec::new();
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-a") {
        list.push(Args::DisableAudio);
    }

    if args.contains("-d") {
        list.push(Args::Debug);
    }

    #[cfg(debug_assertions)]
    if args.contains("-s") {
        list.push(Args::NoSeed);
    }

    list
}