use audio::bind_world_music;
use macroquad::camera::Camera2D;
use macroquad::prelude::Conf;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;

use io::data::configuration::Configuration;
use parking_lot::RwLock;
use scene::loading_scene_manager::LoadingSceneManager;
use scene::scene_manager::SceneManager;
use util::audio::AudioContext;
use util::file::PersistantDataLocation;
use util::text_renderer::TextRenderer;

pub static TITLE: &str =  env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

lazy_static::lazy_static! {
    pub static ref QUIT_SIGNAL: RwLock<bool> = RwLock::new(false);
}

#[macroquad::main(settings)]
async fn main() {
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }
    info!("Starting {}, Version: {}", TITLE, crate::VERSION);
    info!("By {}", crate::AUTHORS);

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    info!("Loading in background...");

    let mut text_renderer = TextRenderer::new();
    let mut scene_manager = SceneManager::default();

    if cfg!(not(target_arch = "wasm32")) {
        storage::store( AudioContext::new());
        bind_world_music();
    }

    let loading_coroutine = macroquad::prelude::coroutines::start_coroutine(load_coroutine());
    
    storage::store(io::data::player::PlayerData::load_async_default().await);


    crate::util::input::default_keybinds();

    while !(*crate::scene::loading_scene_manager::LOADING_SCENE_FINISHED.read()) {
        macroquad::prelude::coroutines::wait_seconds(0.2).await;
    }
    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine);

    text_renderer.default_add();

    scene_manager.load_other_scenes().await;

    info!("Finished loading in background!");

    info!("Starting game!");

    scene_manager.on_start();

    loop {
        scene_manager.input(get_frame_time());
        scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        scene_manager.render(&text_renderer);
        if *QUIT_SIGNAL.read() == true {
            break;
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
    // let tr = TEXT_RENDERER.read();
    loop {
        loading_scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        loading_scene_manager.render();
        macroquad::prelude::next_frame().await;
    }
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


