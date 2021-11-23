use crate::engine::{
    graphics::{self, Color},
    State,
};

use crate::game::{is_debug, set_debug};

use crate::engine::log::info;

use crate::{Args, GameContext};

use super::{game::GameStateManager, menu::MenuStateManager, MainState, MainStates};

pub struct StateManager {
    current: MainStates,

    menu: MenuStateManager,
    game: GameStateManager,
    // scaler: ScreenScaler,
}

impl State<GameContext> for StateManager {
    fn start(&mut self, ctx: &mut GameContext) {
        self.game.load(ctx);

        info!("Finished loading!");
        self.get_mut().start(ctx)
    }
    fn end(&mut self, ctx: &mut GameContext) {
        self.get_mut().end(ctx)
    }
    fn update(&mut self, ctx: &mut GameContext, delta: f32) {
        self.get_mut().update(ctx, delta);
        if let Some(action) = self.get_mut().action() {
            match action {
                super::Action::Goto(state) => {
                    self.get_mut().end(ctx);
                    self.current = state;
                    self.get_mut().start(ctx);
                }
                super::Action::Seed(seed) => self.game.seed(seed),
            }
        }
        // Ok(())
    }
    fn draw(&mut self, ctx: &mut GameContext) {
        // graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        self.get_mut().draw(ctx);
        // graphics::reset_transform_matrix(ctx);
        // graphics::reset_canvas(ctx);
        // graphics::clear(ctx, Color::BLACK);
        // self.scaler.draw(ctx);
        // Ok(())
    }
    // fn event(&mut self, _: &mut GameContext, event: Event) {
    //     if let Event::Resized { width, height } = event {
    //         self.scaler.set_outer_size(width, height);
    //     }

    //     Ok(())
    // }
}
impl StateManager {
    pub fn new(ctx: &mut GameContext, args: Vec<Args>) -> Self {
        // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

        // let texture = game::graphics::Texture::new(include_bytes!("../build/assets/loading.png"));

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

        use crate::engine::{graphics::scaling::*, util};

        let scaler = ScreenScaler::with_size(
            ctx,
            util::WIDTH as _,
            util::HEIGHT as _,
            ScalingMode::Stretch,
        );

        set_scaler(ctx, scaler);

        info!("Loading assets...");

        // Parses arguments

        // let args = getopts();

        {
            if args.contains(&Args::Debug) {
                set_debug(true);
            }

            if is_debug() {
                info!("Running in debug mode");
            }
        }

        // let scaler =
        //     ScreenScaler::with_window_size(ctx, WIDTH as _, HEIGHT as _, ScalingMode::ShowAll)?;

        Self {
            current: Default::default(),
            menu: MenuStateManager::new(ctx),
            game: GameStateManager::new(ctx),
            // scaler,
        }
    }

    fn get_mut(&mut self) -> &mut dyn MainState {
        match self.current {
            MainStates::Menu => &mut self.menu,
            MainStates::Game => &mut self.game,
        }
    }
}
