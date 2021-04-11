#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate firecore_game as game;
extern crate firecore_world as world;
extern crate firecore_battle as battle;

use game::{
    macroquad::{
        Window,
        camera::set_camera,
        prelude::{
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
            warn,
            error,
            draw_rectangle,
        }
    },
    data::{
        get, get_mut, 
        configuration::Configuration,
    },
    util::{
        WIDTH, HEIGHT,
    }
};

use scene::{
    Scene,
    loading::{LOADING_FINISHED, load_coroutine},
    manager::SceneManager,
};

use util::{
    Args,
    loading_screen,
    draw_touch_button
};

use std::sync::atomic::Ordering::Relaxed;

pub mod util;
pub mod scene;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

static mut SCENE_MANAGER: Option<SceneManager> = None;

#[cfg(target_arch = "wasm32")]
#[game::macroquad::main(settings)]
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

    Window::from_config(settings(), start());

    info!("Quitting game...");

    unsafe { SCENE_MANAGER.as_mut().unwrap().quit() };

}

pub async fn start() {

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    set_camera(util::game_camera());
    

    // Loads configuration and player saves

    game::data::store().await;  

    {

        let config = get::<Configuration>().expect("Could not get configuration!");

        game::input::keyboard::load(config.controls.clone());

        if config.touchscreen {
            game::input::touchscreen::touchscreen(true);
        }

    }

    // Loads fonts

    match postcard::from_bytes(include_bytes!("../build/data/fonts.bin")) {
        Ok(font_sheets) => game::init::text(font_sheets),
        Err(err) => {
            warn!("Could not load font sheets with error {}", err);
            warn!("Game will start with no text display.");
        }
    }

    // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

    let texture = game::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));
    
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
        match postcard::from_bytes(include_bytes!("../build/data/audio.bin")) {
            Ok(sound) => game::init::audio(sound),
            Err(err) => error!("Could not read sound file with error {}", err)
        }
        

    }

    {

        if args.contains(&Args::Debug) {
            game::DEBUG.store(true, Relaxed);
        }
        
        if game::is_debug() {
            info!("Running in debug mode");
        }    

    }

    // Load pokedex and movedex;

    match postcard::from_bytes(include_bytes!("../build/data/dex.bin")) {
        Ok(dex) => game::init::pokedex(dex),
        Err(err) => panic!("Could not load pokedex with error {}", err),
    };
   
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

        if let Some(touchscreen) = unsafe { game::input::touchscreen::TOUCHSCREEN.as_ref() } {
            draw_touch_button(&touchscreen.a);
            draw_touch_button(&touchscreen.b);
            draw_touch_button(&touchscreen.up);
            draw_touch_button(&touchscreen.down);
            draw_touch_button(&touchscreen.left);
            draw_touch_button(&touchscreen.right);
        }

        // Toggle debug on key press

        if is_key_pressed(KeyCode::O) {
            let debug = !game::DEBUG.load(Relaxed);
            game::DEBUG.store(debug, Relaxed);
            info!("Debug: {}", debug)
        }

        // Reload configuration on key press

        if is_key_pressed(KeyCode::P) {
            if let Some(mut config) = get_mut::<Configuration>() {
                if let Err(err) = game::data::reload(std::ops::DerefMut::deref_mut(&mut config)).await {
                    warn!("Could not reload configuration with error {}", err);
                }
            }
        }

        // Quit game if asked to

        if game::should_quit() {
            draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, BLACK);
            break;
        }

        next_frame().await;
    }

    scene_manager.quit();

}