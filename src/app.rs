use log::LevelFilter;
use sdl2_window::Sdl2Window as Windower;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use simplelog::WriteLogger;
use opengl_graphics::GlGraphics;
use piston::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateEvent, WindowSettings, EventLoop};
use piston_window::{OpenGL, Context, PistonWindow, clear};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use log::info;

use crate::BASE_HEIGHT;
use crate::BASE_WIDTH;
use crate::TITLE;
use crate::WINDOW_SCALE;
use crate::scene::scene_manager::SceneManager;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;
use crate::util::traits::Loadable;

pub struct Game {

    scene_manager: SceneManager,
    game_context: GameContext,
    text_renderer: TextRenderer,

}

impl Game {

    pub fn new() -> Game {

        if cfg!(debug_assertions) {
            println!("Running in debug mode");
        }

        println!("Starting {}, Version: {}", TITLE, crate::VERSION);
        println!("By {}", crate::AUTHORS);

        Game {
            scene_manager: SceneManager::new(),
            game_context: GameContext::new(),
            text_renderer: TextRenderer::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {

        let scale = crate::CONFIGURATION.lock().unwrap().scale as u32;

        let mut window: PistonWindow<Windower> = WindowSettings::new(
            TITLE,
            [
                BASE_WIDTH * scale,
                BASE_HEIGHT * scale,
            ],
        )
        .resizable(false)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

        window.set_ups(60);

        let mut gl = GlGraphics::new(OpenGL::V3_2);

        self.load()?;
        self.on_start();

        for e in window {
            if let Some(args) = e.update_args() {
                self.scene_manager.input(&mut self.game_context);
                self.key_update();
                self.scene_manager.update(&args, &mut self.game_context);
            }

            if let Some(args) = e.render_args() {
                
                let scale = *WINDOW_SCALE.lock().unwrap();
                // let mut update = crate::UPDATE_WINDOW.lock().unwrap();

                // if *update {
                //     args.window_size = [BASE_WIDTH as f64 * scale as f64, BASE_HEIGHT as f64 * scale as f64];
                //     *update = false;
                // }
                
                let mut ctx = Context::new_abs(
                    args.window_size[0] / scale as f64, 
                    args.window_size[1] / scale as f64
                );

                self.render(&args, &mut ctx, &mut gl);
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

    pub fn render(&mut self, args: &RenderArgs, ctx: &mut Context, g: &mut GlGraphics) {

        g.draw(args.viewport(), |_, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
            self.scene_manager.render(ctx, g, &mut self.text_renderer);
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