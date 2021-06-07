#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate firecore_game as game;

use game::{
    tetra::{Result, ContextBuilder, time::Timestep},
    log::info,
};

use state::StateManager;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

pub mod state;

pub mod args;
use args::*;

fn main() -> Result {

    // Sets up logger

    simple_logger::SimpleLogger::new().init().unwrap();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    let args = args();

    #[cfg(debug_assertions)]
    if args.contains(&Args::Seed) {
        game::init::seed_randoms(std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).map(|dur| dur.as_secs()).unwrap_or_default() % 1000000)
        
    }

    #[cfg(feature = "discord")] 
    game::log::warn!("Discord support is broken!");
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

    game::init::configuration()?;
    
    // Save data in local directory in debug builds
    #[cfg(debug_assertions)]
    game::storage::SAVE_IN_LOCAL_DIRECTORY.store(true, std::sync::atomic::Ordering::Relaxed);

    ContextBuilder::new(TITLE, (game::util::WIDTH * DEFAULT_SCALE) as _, (game::util::HEIGHT * DEFAULT_SCALE) as _)
    .resizable(true)
    .show_mouse(true)
    .timestep(Timestep::Variable)
    .build()?
    .run(|ctx| StateManager::new(ctx, args))?; // to - do: return state

    // #[cfg(feature = "discord")]
    // discord_rich_presence::DiscordIpc::close(&mut client).unwrap();

    Ok(())

}