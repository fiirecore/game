use engine::{
    tetra::{
        graphics::{
            self,
            scaling::{ScalingMode, ScreenScaler},
            Color,
        },
        math::Vec2,
        Event, Result, State,
    },
    util::{HEIGHT, WIDTH},
};

use crate::game::{is_debug, set_debug};

use log::{error, info};

use crate::{Args, GameContext};

use super::{game::GameStateManager, menu::MenuStateManager, MainState, MainStates};

pub struct StateManager<'d> {
    current: MainStates,

    menu: MenuStateManager,
    game: GameStateManager<'d>,

    scaler: ScreenScaler,
}

impl<'d> State<GameContext<'d>> for StateManager<'d> {
    fn begin(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.game.load(ctx);

        info!("Finished loading!");
        self.get_mut().begin(ctx)
    }
    fn end(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.get_mut().end(ctx)
    }
    fn update(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.get_mut().update(ctx)?;
        if let Some(action) = self.get_mut().action() {
            match action {
                super::Action::Goto(state) => {
                    self.get_mut().end(ctx)?;
                    self.current = state;
                    self.get_mut().begin(ctx)?;
                },
                super::Action::Seed(seed) => self.game.seed(seed),
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut GameContext<'d>) -> Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        self.get_mut().draw(ctx)?;
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);
        Ok(())
    }
    fn event(&mut self, _: &mut GameContext<'d>, event: Event) -> Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
impl<'d> StateManager<'d> {
    pub fn new(ctx: &mut GameContext<'d>, args: Vec<Args>) -> Result<Self> {
        // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

        // let texture = game::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));

        // Flash the loading screen once so the screen freezes on this instead of a blank one

        // loading_screen(texture);

        // let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        //     start_coroutine(load_coroutine())
        // } else {
        //     start_coroutine(async move {
        //         loop {
        //             loading_screen(texture);
        //             next_frame().await;
        //         }
        //     })
        // };

        info!("Loading assets...");

        // Parses arguments

        // let args = getopts();

        #[cfg(feature = "audio")]
        if !args.contains(&Args::DisableAudio) {
            //Load audio files and setup audio
            match bincode::deserialize(include_bytes!("../../build/data/audio.bin")) {
                Ok(audio_data) => ctx.engine.audio.init(audio_data),
                Err(err) => error!("Could not read sound file with error {}", err),
            }
        }

        {
            if args.contains(&Args::Debug) {
                set_debug(true);
            }

            if is_debug() {
                info!("Running in debug mode");
            }
        }

        let scaler =
            ScreenScaler::with_window_size(ctx, WIDTH as _, HEIGHT as _, ScalingMode::ShowAll)?;

        Ok(Self {
            current: Default::default(),
            menu: MenuStateManager::new(ctx, scaler.project(Vec2::new(1.0, 1.0))),
            game: GameStateManager::new(ctx),
            scaler,
        })
    }

    fn get_mut(&mut self) -> &mut dyn MainState<'d> {
        match self.current {
            MainStates::Menu => &mut self.menu,
            MainStates::Game => &mut self.game,
        }
    }
}
