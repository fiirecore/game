#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use io::args::Args;
use firecore_data::configuration::Configuration;
use macroquad::camera::Camera2D;
use macroquad::prelude::BLACK;
use macroquad::prelude::Conf;
use macroquad::prelude::clear_background;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;
use macroquad::prelude::next_frame;
use macroquad::prelude::coroutines::start_coroutine;
use parking_lot::Mutex;
use scene::loading::manager::load_coroutine;
use scene::manager::SceneManager;
use firecore_data::data::PersistantDataLocation;
use scene::Scene;
use util::graphics::draw_text_left;

pub mod util;
pub mod scene;
pub mod io;
pub mod world;
pub mod battle;
pub mod gui;

pub mod experimental;

pub mod pokemon;

pub static TITLE: &str = "Pokemon FireRed";
pub static DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static SCALE: f32 = 3.0;

static mut QUIT: bool = false;

lazy_static::lazy_static! {
    static ref SCENE_MANAGER: Mutex<SceneManager> = Mutex::new(SceneManager::new());
}

#[cfg(target_arch = "wasm32")]
#[macroquad::main(settings)]
async fn main() {
    macroquad_main().await;
}


#[cfg(not(target_arch = "wasm32"))]
fn main() {

    macroquad::Window::from_config(settings(), macroquad_main());

    info!("Quitting game...");

    unsafe{SCENE_MANAGER.force_unlock();}
    SCENE_MANAGER.lock().quit();

}

async fn macroquad_main() {

    let texture = crate::util::graphics::texture::byte_texture(include_bytes!("../build/assets/loading.png"));
    clear_background(macroquad::prelude::BLUE);
    macroquad::prelude::draw_texture(texture, 0.0, 0.0, macroquad::prelude::WHITE);
    draw_text_left(0, VERSION, 1.0, 1.0);
    draw_text_left(1, "The game may stay on this screen", 5.0, 50.0);
    draw_text_left(1, "for up to two minutes.", 5.0, 65.0);
    next_frame().await;

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    #[cfg(debug_assertions)] {
        info!("Running in debug mode");
    }

    let config = Configuration::load_from_file().await;

    config.on_reload();

    storage::store(config);

    crate::util::text::load().await;

    let args = crate::io::args::parse_args();

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    if !args.contains(&Args::DisableAudio) {
        firecore_audio::create();
    }

    let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        start_coroutine(load_coroutine())
    } else {
        start_coroutine(async move {
            loop {
                clear_background(macroquad::prelude::BLUE);
                macroquad::prelude::draw_texture(texture, 0.0, 0.0, macroquad::prelude::WHITE);
                draw_text_left(1, &format!("v{}", VERSION), 2.0, 0.0);
            	draw_text_left(1, "The game may stay on this screen", 5.0, 50.0);
            	draw_text_left(1, "for up to two minutes.", 5.0, 65.0);
                next_frame().await;
            }
        })
    };

    info!("Loading assets...");

    firecore_audio::bind_world_music().await;
    
    pokemon::load().await;

    let mut scene_manager = SCENE_MANAGER.lock();

    scene_manager.load_all().await;

    info!("Finished loading assets!");

    if cfg!(not(target_arch = "wasm32")) {
        while !loading_coroutine.is_done() {
            macroquad::prelude::coroutines::wait_seconds(0.05).await;
        } 
    }

    macroquad::prelude::coroutines::stop_coroutine(loading_coroutine); 

    if cfg!(target_arch = "wasm32") {
        load_coroutine().await;
    }  

    info!("Starting game!");

    scene_manager.on_start().await;

    loop {

        #[cfg(target_arch = "wasm32")]
        firecore_audio::backend::quadsnd::music::MIXER.lock().frame();


        scene_manager.input(get_frame_time());
        
        scene_manager.poll(get_frame_time()).await;


        clear_background(BLACK);

        scene_manager.render();
        // io::input::touchscreen::TOUCH_CONTROLS.render();


        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F12) {
            if let Some(mut config) = storage::get_mut::<Configuration>() {
                firecore_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut config)).await; // maybe change into coroutine
            }
            if let Some(mut player_data) = storage::get_mut::<crate::io::data::player::PlayerData>() {
                firecore_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut player_data)).await;
            }
        }

        if unsafe{QUIT} {
            break;
        }

        next_frame().await;
    }

    scene_manager.quit();

}

pub fn queue_quit() {
    unsafe {
        QUIT = true;
    }
}

fn settings() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (BASE_WIDTH * SCALE as u32) as _,
        window_height: (BASE_HEIGHT * SCALE as u32) as _,
        sample_count: 1,
        ..Default::default()
    }
}