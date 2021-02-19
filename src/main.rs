use io::args::Args;
use io::data::configuration::Configuration;
use macroquad::camera::Camera2D;
use macroquad::prelude::BLACK;
use macroquad::prelude::Conf;
use macroquad::prelude::clear_background;
use macroquad::prelude::collections::storage;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::info;
use macroquad::prelude::next_frame;
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
pub mod gui;

pub static TITLE: &str = "Pokemon FireRed";
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static SCALE: f32 = 3.0;

pub static SAVEABLE: bool = cfg!(not(target_arch = "wasm32"));

static mut QUIT: bool = false;


#[macroquad::main(settings)]
async fn main() {

    //crate::io::embed::create_root_dir();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);
    
    if cfg!(debug_assertions) {
        info!("Running in debug mode");
    }

    let config = Configuration::load_from_file().await;

    config.on_reload();

    storage::store(config);

    let args = crate::io::args::parse_args();

    macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, BASE_WIDTH as _, BASE_HEIGHT as _)));

    if !args.contains(&Args::DisableAudio) {
        #[cfg(not(target_arch = "wasm32"))]
        storage::store(crate::audio::kira::context::AudioContext::new());
        #[cfg(target_arch = "wasm32")]
        crate::audio::quadsnd::bind_gamefreak().await;
    }

    let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        macroquad::prelude::coroutines::start_coroutine(load_coroutine())
    } else {
        macroquad::prelude::coroutines::start_coroutine(async {
            let texture = crate::util::graphics::texture::byte_texture(include_bytes!("../build/assets/loading.png"));
            loop {
                clear_background(macroquad::prelude::BLUE);
                macroquad::prelude::draw_texture(texture, 0.0, 0.0, macroquad::prelude::WHITE);
                next_frame().await;
            }
        })
    };

    info!("Loading assets...");

    let mut scene_manager = SceneManager::default();
    
    audio::bind_world_music().await;
    
    storage::store(io::data::player::PlayerData::load_from_file().await);

    scene_manager.load_other_scenes().await;

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

    scene_manager.on_start();

    loop {

        #[cfg(target_arch = "wasm32")]
        crate::audio::quadsnd::context::music::MIXER.lock().frame();

        // crate::io::input::controller::update_active_controls();
    
        // You can also use cached gamepad state

        scene_manager.input(get_frame_time());
        
        scene_manager.update(get_frame_time());


        clear_background(BLACK);

        scene_manager.render();
        io::input::touchscreen::TOUCH_CONTROLS.render();


        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F12) {
            if let Some(mut config) = storage::get_mut::<Configuration>() {
                util::file::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut config)).await; // maybe change into coroutine
            }
        }

        if unsafe{QUIT} {
            break;
        }

        next_frame().await;
    }

    info!("Quitting game...");
    util::Quit::quit(&mut scene_manager);

}

pub fn queue_quit() {
    unsafe {
        QUIT = true;
    }
}

fn settings() -> Conf {
    // let config = Configuration::load_from_file();
    // let scale = config.window_scale as u32;
    //storage::store(config);

    Conf {
        window_title: TITLE.to_string(),
        window_width: (BASE_WIDTH * SCALE as u32) as _,
        window_height: (BASE_HEIGHT * SCALE as u32) as _,
        sample_count: 1,
        ..Default::default()
    }
}