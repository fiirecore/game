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
    is_key_pressed,
    KeyCode,
};

use firecore_data::{
    get, get_mut, 
    configuration::Configuration,
};

use scene::{
    Scene,
    loading::{LOADING_FINISHED, load_coroutine},
    manager::SceneManager,
};

use util::{
    Args,
    loading_screen,
    graphics::draw_touch_button
};

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub mod util;
pub mod scene;
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

static DEBUG: AtomicBool = AtomicBool::new(cfg!(debug_assertions));
static QUIT: AtomicBool = AtomicBool::new(false);

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

    firecore_data::store().await;  

    {

        let config = get::<Configuration>().expect("Could not get configuration!");

        firecore_input::keyboard::load(config.controls.clone());

        if config.touchscreen {
            firecore_input::touchscreen::touchscreen(true);
        }

    }

    // Loads fonts

    crate::util::text::init_text().await;  

    // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

    let texture = crate::util::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));
    
    // Flash the loading screen once so the screen freezes on this instead of a blank one

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

    #[cfg(feature = "audio")]
    if !args.contains(&Args::DisableAudio) {

        // Load audio files and setup audio

        util::load_audio().await;

    }

    {

        if args.contains(&Args::Debug) {
            DEBUG.store(true, Relaxed);
        }
        
        if debug() {
            info!("Running in debug mode");
        }    

    }

   
    unsafe { SCENE_MANAGER = Some(SceneManager::new()) };

    let scene_manager = unsafe { SCENE_MANAGER.as_mut().unwrap() };

    scene_manager.load().await;

    info!("Finished loading assets!");
    LOADING_FINISHED.store(true, Relaxed);


    // Wait for the loading scenes to finish, then stop the coroutine

    #[cfg(not(target_arch = "wasm32"))] {
        while !loading_coroutine.is_done() {
            wait_seconds(0.1).await;
        } 
    }

    stop_coroutine(loading_coroutine); 

    // Start the loading scenes on wasm32 because they lag in a coroutine

    #[cfg(target_arch = "wasm32")] {
        load_coroutine().await;
    }

    info!("Starting game!");

    scene_manager.on_start();

    loop {

        #[cfg(all(target_arch = "wasm32", feature = "audio"))]
        firecore_audio::backend::quadsnd::music::MIXER.lock().frame();


        scene_manager.input(get_frame_time());
        
        scene_manager.update(get_frame_time());


        clear_background(BLACK);

        scene_manager.render();
        // scene_manager.ui();

        // Render touchscreen controls if they are active

        if let Some(touchscreen) = unsafe { firecore_input::touchscreen::TOUCHSCREEN.as_ref() } {
            draw_touch_button(&touchscreen.a);
            draw_touch_button(&touchscreen.b);
            draw_touch_button(&touchscreen.up);
            draw_touch_button(&touchscreen.down);
            draw_touch_button(&touchscreen.left);
            draw_touch_button(&touchscreen.right);
        }

        // Toggle debug on key press

        if is_key_pressed(KeyCode::O) {
            let debug = !DEBUG.load(Relaxed);
            DEBUG.store(debug, Relaxed);
            info!("Debug: {}", debug)
        }

        // Reload configuration on key press

        if is_key_pressed(KeyCode::P) {
            if let Some(mut config) = get_mut::<Configuration>() {
                if let Err(err) = firecore_data::reload(std::ops::DerefMut::deref_mut(&mut config)).await {
                    macroquad::prelude::warn!("Could not reload configuration with error {}", err);
                }
            }
        }

        // Quit game if asked to

        if QUIT.load(Relaxed) {
            util::graphics::draw_rect(BLACK, 0.0, 0.0, WIDTH, HEIGHT);
            break;
        }

        next_frame().await;
    }

    scene_manager.quit();

}

pub fn quit() {
    QUIT.store(true, Relaxed);
}

pub fn debug() -> bool {
    DEBUG.load(Relaxed)
}