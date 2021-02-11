use io::args::Args;
use macroquad::camera::Camera2D;
use macroquad::prelude::Conf;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;
use parking_lot::Mutex;
use scene::loading_scene_manager::load_coroutine;
use scene::scene_manager::SceneManager;
use util::file::PersistantDataLocation;

pub mod util;
pub mod audio;
pub mod scene;
pub mod entity;
pub mod io;
pub mod world;
pub mod pokemon;
pub mod game;
pub mod battle;
mod gui;

pub static TITLE: &str =  env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static SAVEABLE: bool = cfg!(not(target_arch = "wasm32"));

lazy_static::lazy_static!{
    static ref QUIT: Mutex<bool> = Mutex::new(false);
}
#[macroquad::main(settings)]
async fn main() {

    //crate::io::embed::create_root_dir();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);
    
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }

    let args = crate::io::args::parse_args();

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    if !args.contains(&Args::DisableAudio) {
        #[cfg(feature = "audio")]
        storage::store(crate::audio::context::AudioContext::new());
    }

    let loading_coroutine = macroquad::prelude::coroutines::start_coroutine(load_coroutine());


    info!("Loading in background...");

    let mut scene_manager = SceneManager::default();
    
    #[cfg(feature = "audio")]
    audio::loader::bind_world_music();
    
    storage::store(io::data::player::PlayerData::load_from_file().await);

    scene_manager.load_other_scenes().await;

    info!("Finished loading in background!");



    while !loading_coroutine.is_done() {
        macroquad::prelude::coroutines::wait_seconds(0.05).await;
    }
    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine);   

    info!("Starting game!");

    scene_manager.on_start();

    loop {
        scene_manager.input(get_frame_time());
        scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        scene_manager.render();
        if *QUIT.lock() {
            break;
        }
        macroquad::prelude::next_frame().await;
    }

    info!("Quitting game...");
    util::Quit::quit(&mut scene_manager);

}

pub fn queue_quit() {
    *QUIT.lock() = true;
    //*RUNNING.write() = false;
}

fn settings() -> Conf {
    // let config = Configuration::load_from_file();
    // let scale = config.window_scale as u32;
    let scale = 3;
    //storage::store(config);

    Conf {
        window_title: TITLE.to_string(),
        window_width: (BASE_WIDTH * scale) as _,
        window_height: (BASE_HEIGHT * scale) as _,
        sample_count: 0,
        ..Default::default()
    }
}


