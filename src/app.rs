use log::LevelFilter;
use sdl2_window::Sdl2Window as Windower;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use simplelog::WriteLogger;
use opengl_graphics::GlGraphics;
use piston::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings, EventLoop};
use piston_window::{OpenGL, Context, PistonWindow, clear};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use log::info;

use crate::scene::scene_manager::SceneManager;
use crate::engine::text::TextRenderer;
use crate::engine::game_context::GameContext;
use crate::util::traits::Loadable;

pub struct Game {

    scene_manager: SceneManager,
    game_context: GameContext,
    text_renderer: TextRenderer,

}

pub static WIDTH: usize = 240; //*SCALE;
pub static HEIGHT: usize = 160; //*SCALE;
                                //pub static SCALE: usize = 1;
                                //pub static TILE_SIZE: usize = 16;
impl Game {

    pub fn new() -> Game {

        if cfg!(debug_assertions) {
            println!("Running in debug mode");
        }

        let configuration = crate::io::data::configuration::Configuration::load();
        println!(
        "Starting {}, Version: {}",
        configuration.name, configuration.version
        );
        println!("By {}", configuration.authors);
        

        Game {
            scene_manager: SceneManager::new(),
            game_context: GameContext::new(configuration),
            text_renderer: TextRenderer::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {

        let mut window: PistonWindow<Windower> = WindowSettings::new(
            &self.game_context.configuration.name,
            [
                (self.game_context.configuration.width * self.game_context.configuration.scale) as u32,
                (self.game_context.configuration.height * self.game_context.configuration.scale) as u32,
            ],
        )
        .resizable(false)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

        window.set_ups(60);

        let mut gl = GlGraphics::new(OpenGL::V3_2);

        //let sdl = window.window.sdl_context.to_owned();

        self.load()?;
        self.on_start();

        for e in window {
            if let Some(args) = e.update_args() {
                self.update(&args);
            }

            if let Some(ref args) = e.render_args() {
                self.render(args, &mut gl);
            }

            if let Some(ref args) = e.press_args() {
                self.key_press(args);
            }

            if let Some(ref args) = e.release_args() {
                self.key_release(args);
            }
        }

        self.dispose();

        Ok(())
    }
    

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        println!("Loading...");

        let mut log_name = String::from("logs/log_");
        log_name.push_str(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string().as_str());
        log_name.push_str(".txt");

        if !PathBuf::from("logs").exists() {
            std::fs::create_dir("logs").expect("Could not create logs directory!");
        }
        
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
                WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_name).unwrap()),
            ]
        )?;

        self.text_renderer.load_textures();

        self.text_renderer.default_add();

        self.scene_manager.load();

        let keys = vec![Key::X, Key::Z, Key::Up, Key::Down, Key::Left, Key::Right, Key::Escape, Key::LShift];

        self.game_context.fill_keymaps(keys);

        info!("Done loading!");

        Ok(())
    }

    pub fn on_start(&mut self) {
        self.scene_manager.on_start(&mut self.game_context);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // if self.game_context.fkeys[2] == 1 {
        //     self.game_context.app_console.toggle();
        // }
        self.scene_manager.input(&mut self.game_context);
        self.key_update();
        self.scene_manager.update(args, &mut self.game_context);
    }

    pub fn render(&mut self, args: &RenderArgs, g: &mut GlGraphics) {
        
        let mut ctx = Context::new_abs(args.window_size[0]/self.game_context.configuration.scale as f64, args.window_size[1]/self.game_context.configuration.scale as f64);

        g.draw(args.viewport(), |_, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
            self.scene_manager.render(&mut ctx, g, &mut self.text_renderer);
            //self.game_context.app_console.render(&mut ctx, g, &mut self.text_renderer);
        });

    }

    pub fn key_press(&mut self, button: &Button) {
        if let Some(i) = self.game_context.keymap.get(button) {
            if i < &self.game_context.keys.len() {
                self.game_context.keys[*i] = 1;
            }
        }
        if let Some(i) = self.game_context.fkeymap.get(button) {
            if i < &self.game_context.fkeys.len() {
                self.game_context.fkeys[*i] = 1;
            }
        }
    }

    pub fn key_release(&mut self, button: &Button) {
        if let Some(i) = self.game_context.keymap.get(button) {
            if i < &self.game_context.keys.len() {
                self.game_context.keys[*i] = 3;
            }
        }
        if let Some(i) = self.game_context.fkeymap.get(button) {
            if i < &self.game_context.fkeys.len() {
                self.game_context.fkeys[*i] = 3;
            }
        }
    }

    pub fn key_update(&mut self) {
        for i in 0..self.game_context.keys.len() {
            if self.game_context.keys[i] == 1 {
                self.game_context.keys[i] = 2;
            }
        }

        for i in 0..self.game_context.fkeys.len() {
            if self.game_context.fkeys[i] == 1 {
                self.game_context.fkeys[i] = 2;
            }
        }

        for i in 0..self.game_context.keys.len() {
            if self.game_context.keys[i] == 3 {
                self.game_context.keys[i] = 0;
            }
        }

        for i in 0..self.game_context.fkeys.len() {
            if self.game_context.fkeys[i] == 3 {
                self.game_context.fkeys[i] = 0;
            }
        }
    }

    pub fn dispose(&mut self) {
        info!("Closing app...");
        self.scene_manager.dispose();
    }
}

/*

impl emscripten_main_loop::MainLoop for Game {

    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    self.dispose();
                    return emscripten_main_loop::MainLoopEvent::Terminate;
                }
                Event::KeyDown { keycode, .. } => {
                    self.poll_key_down(keycode);
                    if keycode == Some(Keycode::Escape) {
                        self.dispose();
                        return emscripten_main_loop::MainLoopEvent::Terminate;
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    self.poll_key_up(keycode);
                },
                _ => {}

            }

        }

        self.key_update();

        self.scene_manager.update(&mut self.game_context);

        self.canvas.clear();
        self.scene_manager.render(&self.canvas, &self.text_renderer);
        self.game_context.app_console.render(&self.canvas, &self.text_renderer);
        self.canvas.present();

        emscripten_main_loop::MainLoopEvent::Continue

    }

}

*/
