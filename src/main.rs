#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::{
    Conf,
    clear_background,
    BLACK,
    get_frame_time,
    next_frame,
    info,
    coroutines::{
        start_coroutine,
        stop_coroutine,
        wait_seconds,
    },
};

use firecore_data::{get, get_mut, configuration::Configuration};

use scene::{
    Scene,
    loading::manager::load_coroutine,
    manager::SceneManager,
};

use util::{
    Args,
    loading_screen,
    graphics::draw_touch_button
};

pub mod util;
pub mod scene;
pub mod data;
pub mod world;
pub mod battle;
pub mod gui;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const WIDTH: f32 = 240.0;
pub const HEIGHT: f32 = 160.0;
pub const DEFAULT_SCALE: f32 = 3.0;

static mut DEBUG: bool = cfg!(debug_assertions);
static mut QUIT: bool = false;

static mut SCENE_MANAGER: Option<SceneManager> = None;

#[cfg(target_arch = "wasm32")]
#[macroquad::main(settings)]
async fn main() {
    start().await;
}

fn settings() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (WIDTH * DEFAULT_SCALE) as _,
        window_height: (HEIGHT * DEFAULT_SCALE) as _,
        sample_count: 1,
        ..Default::default()
    }
}


#[cfg(not(target_arch = "wasm32"))]
fn main() {

    macroquad::Window::from_config(settings(), start());

    info!("Quitting game...");

    unsafe { SCENE_MANAGER.as_mut().unwrap().quit() };

}

pub async fn start() {

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    macroquad::camera::set_camera(util::game_camera());
    

    // Loads configuration and player saves

    firecore_data::load().await;  

    {

        let config = get::<Configuration>().unwrap();

        firecore_input::keyboard::load(firecore_input::keyboard::serialization::normal_map(&config.controls));

        if config.touchscreen {
            firecore_input::touchscreen::touchscreen(true);
        }

    }

     

    // Loads fonts

    crate::util::text::init_text().await;  

    // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

    let texture = crate::util::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));
    
    loading_screen(texture);

    let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        start_coroutine(load_coroutine())
    } else {
        start_coroutine(async move {
            loop {
                loading_screen(texture);
                next_frame().await;
            }
        })
    };

    info!("Loading assets...");

    // Parses arguments

    let args = util::getopts();

    if !args.contains(&Args::DisableAudio) {

        // Load audio files and setup audio

        #[cfg(feature = "audio")]
        if let Err(err) = firecore_audio::create() {
            macroquad::prelude::error!("Could not create audio instance with error {}", err);
        } else {
            let audio = bincode::deserialize(
                // &macroquad::prelude::load_file("assets/audio.bin").await.unwrap()
                include_bytes!("../build/data/audio.bin")
            ).unwrap();

            #[cfg(not(target = "wasm32"))] {
                std::thread::spawn( || {
                    if let Err(err) = firecore_audio::load(audio) {
                        macroquad::prelude::error!("Could not load audio files with error {}", err);
                    }
                });
            }
            #[cfg(target = "wasm32")] {
                if let Err(err) = firecore_audio::load(audio) {
                    macroquad::prelude::error!("Could not load audio files with error {}", err);
                }
            }

        }

    }

    if args.contains(&Args::Debug) {
        unsafe { DEBUG = true; }
    }
    
    if debug() {
        info!("Running in debug mode");
    }

    unsafe { SCENE_MANAGER = Some(SceneManager::new()) };

    let scene_manager = unsafe { SCENE_MANAGER.as_mut().unwrap() };
    
    // Load the pokedex, pokemon textures and moves

    util::pokemon::load(&mut scene_manager.game_scene.pokemon_textures).await;

    scene_manager.load_all().await;

    info!("Finished loading assets!");


    if cfg!(not(target_arch = "wasm32")) {
        while !loading_coroutine.is_done() {
            wait_seconds(0.05).await;
        } 
    }

    stop_coroutine(loading_coroutine); 

    if cfg!(target_arch = "wasm32") {
        load_coroutine().await;
    }

    info!("Starting game!");

    scene_manager.on_start().await;

    loop {

        #[cfg(all(target_arch = "wasm32", feature = "audio"))]
        firecore_audio::backend::quadsnd::music::MIXER.lock().frame();


        scene_manager.input(get_frame_time());
        
        scene_manager.poll(get_frame_time()).await;


        clear_background(BLACK);

        scene_manager.render();
        scene_manager.ui();

        if let Some(touchscreen) = unsafe { firecore_input::touchscreen::TOUCHSCREEN.as_ref() } {
            draw_touch_button(&touchscreen.a);
            draw_touch_button(&touchscreen.b);
            draw_touch_button(&touchscreen.up);
            draw_touch_button(&touchscreen.down);
            draw_touch_button(&touchscreen.left);
            draw_touch_button(&touchscreen.right);
        }

        // io::input::touchscreen::TOUCH_CONTROLS.render();


        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F12) {
            if let Some(mut config) = get_mut::<Configuration>() {
                firecore_data::data::PersistantData::reload(std::ops::DerefMut::deref_mut(&mut config)).await; // maybe change into coroutine
            }
        }

        if unsafe{QUIT} {
            util::graphics::draw_rect(BLACK, 0.0, 0.0, WIDTH, HEIGHT);
            break;
        }

        next_frame().await;
    }

    scene_manager.quit();

}

pub fn quit() {
    unsafe {
        QUIT = true;
    }
}

pub fn debug() -> bool {
    unsafe{DEBUG}
}