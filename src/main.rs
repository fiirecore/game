use macroquad::camera::Camera2D;
use macroquad::prelude::Conf;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;
use parking_lot::Mutex;

use io::data::configuration::Configuration;
use scene::loading_scene_manager::LoadingSceneManager;
use scene::scene_manager::SceneManager;
use util::text_renderer::TextRenderer;

pub static TITLE: &str =  env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

lazy_static::lazy_static! {
    pub static ref CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::saved_default());
    //pub static ref TEXT_RENDERER: RwLock<TextRenderer> = RwLock::new(TextRenderer::new());
    //pub static ref RUNNING: parking_lot::RwLock<bool> = parking_lot::RwLock::new(true);
}

#[macroquad::main(settings)]
async fn main() {
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }
    info!("Starting {}, Version: {}", TITLE, crate::VERSION);
    info!("By {}", crate::AUTHORS);

    info!("Loading...");

    let camera = Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _));
    macroquad::camera::set_camera(camera);

    let loading_coroutine = macroquad::prelude::coroutines::start_coroutine(load_coroutine());

    let mut text_renderer = TextRenderer::new();
    text_renderer.default_add();

    let mut scene_manager = SceneManager::default();    

    scene_manager.load_other_scenes().await;

    crate::util::input::default_keybinds();
    scene_manager.on_start();

    info!("Finished loading game in background!");

    while !(*crate::scene::loading_scene_manager::LOADING_SCENE_FINISHED.read()) {
        macroquad::prelude::coroutines::wait_seconds(2.0).await;
    }

    info!("Starting title screen");

    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine);

    loop {
        scene_manager.input(get_frame_time());
        scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        scene_manager.render(&text_renderer);
        macroquad::prelude::next_frame().await;
    }

}

fn settings() -> Conf {
    let scale = 3;
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
    let mut loading_scene_manager = LoadingSceneManager::new().await;
    // let tr = TEXT_RENDERER.read();
    loop {
        loading_scene_manager.update(get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        loading_scene_manager.render();
        macroquad::prelude::next_frame().await;
    }
}

//pub mod app;

pub mod util;

pub mod audio;

pub mod scene;

pub mod entity;

pub mod io;

pub mod world;

pub mod game {

    pub mod game_manager;

    pub mod player_data_container;

    pub mod pokedex {
        pub mod pokedex;
        pub mod pokemon {
            pub mod pokemon_instance;
            pub mod pokemon_owned;
        }
        pub mod move_instance;
    }

}

pub mod battle;

mod gui {

    pub mod gui;

    pub mod basic_button;

    pub mod battle {

        pub mod battle_gui;

        pub mod battle_background;
        pub mod health_bar;
        pub mod pokemon_gui;
        pub mod battle_text;
        pub mod player_bounce;

        pub mod panels {
            pub mod player_panel;
            pub mod battle_panel;
            pub mod fight_panel;
            pub mod move_panel;
        }
    
    }

    pub mod game {
        pub mod pokemon_party_gui;
    }

}


