use data::player::list::PlayerSaves;
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use firecore_data::configuration::Configuration;
use macroquad::camera::Camera2D;
use macroquad::prelude::BLACK;
use macroquad::prelude::Conf;
use macroquad::prelude::Rect;
use macroquad::prelude::clear_background;
use macroquad::prelude::collections::storage;
use macroquad::prelude::error;
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
pub mod data;
pub mod world;
pub mod battle;
pub mod gui;

pub mod experimental;

pub mod pokemon;
pub mod audio;

pub static TITLE: &str = "Pokemon FireRed";
pub static DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

pub static WIDTH_F32: f32 = crate::BASE_WIDTH as f32;
pub static HEIGHT_F32: f32 = crate::BASE_HEIGHT as f32;

pub static CAMERA_SIZE: Rect = Rect { x: 0.0, y: 0.0, w: WIDTH_F32, h: HEIGHT_F32 };

pub static SCALE: f32 = 3.0;

static mut DEBUG: bool = cfg!(debug_assertions);
static mut QUIT: bool = false;

lazy_static::lazy_static! {
    static ref SCENE_MANAGER: Mutex<SceneManager> = Mutex::new(SceneManager::new());
}

#[cfg(target_arch = "wasm32")]
#[macroquad::main(settings)]
async fn main() {
    macroquad_main().await;
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


#[cfg(not(target_arch = "wasm32"))]
fn main() {

    macroquad::Window::from_config(settings(), macroquad_main());

    info!("Quitting game...");

    unsafe{SCENE_MANAGER.force_unlock();}
    SCENE_MANAGER.lock().quit();

}

async fn macroquad_main() {

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    macroquad::camera::set_camera(Camera2D::from_display_rect(CAMERA_SIZE));

    let texture = crate::util::graphics::texture::byte_texture(include_bytes!("../build/assets/loading.png"));
    clear_background(macroquad::prelude::BLUE);
    macroquad::prelude::draw_texture(texture, 0.0, 0.0, macroquad::prelude::WHITE);
    draw_text_left(0, VERSION, 1.0, 1.0);
    draw_text_left(1, "The game may stay on this screen", 5.0, 50.0);
    draw_text_left(1, "for up to two minutes.", 5.0, 65.0);
    next_frame().await;

    let config = Configuration::load_from_file().await;

    config.on_reload();

    let saves = PlayerSaves::load_from_file().await;

    storage::store(saves);

    storage::store(config);

    crate::data::text::font::open_sheets().await;

    let args = getopts();

    if !args.contains(&Args::DisableAudio) {
        if let Err(err) = firecore_audio::create() {
            error!("Could not create audio instance with error {}", err);
        }
    }

    if args.contains(&Args::Debug) {
        unsafe { DEBUG = true; }
    }
    
    if debug() {
        info!("Running in debug mode");
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

    let audio = bincode::deserialize(
    // &macroquad::prelude::load_file("assets/audio.bin").await.unwrap()
        include_bytes!("../assets/audio.bin")
    ).unwrap();
    #[cfg(not(target = "wasm32"))] {
        std::thread::spawn( || {
            if let Err(err) = firecore_audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        });
    }
    #[cfg(target = "wasm32")]
    if let Err(err) = firecore_audio::load(audio) {
        error!("Could not load audio files with error {}", err);
    }
    
    pokemon::load().await;

    let mut scene_manager = SCENE_MANAGER.lock();

    scene_manager.load_all().await;

    // let mut egui_mq = egui_miniquad::EguiMq::new(unsafe{macroquad::window::get_internal_gl().quad_context});

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
        scene_manager.ui();
        // io::input::touchscreen::TOUCH_CONTROLS.render();


        // if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F12) {
        //     if let Some(mut config) = storage::get_mut::<Configuration>() {
        //         firecore_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut config)).await; // maybe change into coroutine
        //     }
        //     if let Some(player_data) = crate::io::data::player::PLAYER_DATA.write().as_mut() {
        //         firecore_data::data::PersistantData::reload(player_data).await;
        //     }
        // }

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

pub fn debug() -> bool {
    unsafe{DEBUG}
}

#[derive(PartialEq)]
pub enum Args {

    DisableAudio,
    Debug,

}

fn getopts() -> Vec<Args> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    let mut list = Vec::new();

    opts.optflag("a", "disable-audio", "Disable audio");
    opts.optflag("d", "debug", "Add debug keybinds and other stuff");

    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("a") {
                list.push(Args::DisableAudio);
            }
            if m.opt_present("d") {
                list.push(Args::Debug);
            }
        }
        Err(f) => {
            macroquad::prelude::warn!("Could not parse command line arguments with error {}", f.to_string());
        }
    };
    return list;
}