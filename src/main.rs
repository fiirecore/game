use macroquad::camera::Camera2D;
use macroquad::prelude::Conf;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;

use io::data::configuration::Configuration;
use macroquad::prelude::warn;
use scene::loading_scene_manager::LoadingSceneManager;
use scene::scene_manager::SceneManager;
use util::file::PersistantDataLocation;

pub static TITLE: &str =  env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static mut RUNNING: bool = true;

#[macroquad::main(settings)]
async fn main() {
    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);
    
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }

    let args: Vec<String> = std::env::args().collect();

    let mut load_music = true;

    let mut opts = getopts::Options::new();
    opts.optflag("m", "no-music", "Disable music");
    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("m") {
                load_music = false;
            }
        }
        Err(f) => {
            warn!("Could not parse command line arguments with error {}", f.to_string());
        }
    };

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    info!("Loading in background...");

    if load_music {
        storage::store(util::audio::AudioContext::new());
    }

    let loading_coroutine = macroquad::prelude::coroutines::start_coroutine(load_coroutine());

    let mut scene_manager = SceneManager::default();
    audio::loader::bind_world_music();
    
    storage::store(io::data::player::PlayerData::load_async_default().await);

    while !loading_coroutine.is_done() {
        macroquad::prelude::coroutines::wait_seconds(0.2).await;
    }
    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine);

    scene_manager.load_other_scenes().await;

    info!("Finished loading in background!");

    info!("Starting game!");

    scene_manager.on_start();

    loop {
        scene_manager.input(get_frame_time());
        scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        scene_manager.render();
        unsafe {
            if !RUNNING {
                break;
            }
        }
        macroquad::prelude::next_frame().await;
    }

    util::Quit::quit(&mut scene_manager);

}

fn settings() -> Conf {
    let config = Configuration::load_from_file();
    let scale = config.window_scale as u32;
    storage::store(config);

    Conf {
        window_title: TITLE.to_string(),
        window_width: (BASE_WIDTH * scale) as _,
        window_height: (BASE_HEIGHT * scale) as _,
        sample_count: 0,
        ..Default::default()
    }
}

fn not_debug() -> bool {
    !cfg!(debug_assertions) || cfg!(target_arch = "wasm32")
}

async fn load_coroutine() {
    info!("Starting loading scene coroutine");
    let mut loading_scene_manager = LoadingSceneManager::new();
    while !loading_scene_manager.finished {
        loading_scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        loading_scene_manager.render();
        macroquad::prelude::next_frame().await;
    }
}

pub enum RunState {

    Continue,
    Quit,

}

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


