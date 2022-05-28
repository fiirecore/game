mod save;

use std::{ops::Deref, rc::Rc};

use event::{EventReader, EventWriter};
use worldcli::engine::notan::AppState;

use crate::{
    engine::{
        controls::{pressed, Control},
        graphics::{Color, CreateDraw, CreateFont, DrawShapes, Graphics},
        App, Plugins,
    },
    load::LoadData,
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    saves::Player,
    state::MainStates,
    touchscreen::Touchscreen,
};

use super::{
    console::Console, game::GameStateManager, loading::LoadingStateManager,
    menu::title::TitleState, StateMessage,
};

use crate::engine::notan;

#[derive(AppState)]
pub struct StateManager<
    P: Deref<Target = Pokemon> + Clone + From<Pokemon> + std::fmt::Debug = Rc<Pokemon>,
    M: Deref<Target = Move> + Clone + From<Move> + std::fmt::Debug = Rc<Move>,
    I: Deref<Target = Item> + Clone + From<Item> + std::fmt::Debug = Rc<Item>,
> {
    state: MainStates,

    loading: LoadingStateManager,
    title: TitleState,
    game: GameStateInit<P, M, I>,

    console: Console,

    save: save::PlayerSave<P, M, I>,

    data: crate::load::LoadData<P, M, I>,

    sender: EventWriter<StateMessage>,
    receiver: EventReader<StateMessage>,
}

enum GameStateInit<
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    Uninit(
        worldcli::engine::graphics::Font,
        event::EventWriter<StateMessage>,
        crate::command::CommandProcessor,
        u8,
    ),
    Init(GameStateManager<P, M, I>),
    None,
}

impl<
        P: Deref<Target = Pokemon> + Clone + From<Pokemon> + std::fmt::Debug,
        M: Deref<Target = Move> + Clone + From<Move> + std::fmt::Debug,
        I: Deref<Target = Item> + Clone + From<Item> + std::fmt::Debug,
    > StateManager<P, M, I>
{
    pub(crate) fn new(gfx: &mut Graphics, load: LoadData<P, M, I>) -> Self {
        Self::try_new(gfx, load)
            .unwrap_or_else(|err| panic!("Could not create state manager with error {}", err))
    }

    fn try_new(gfx: &mut Graphics, load: LoadData<P, M, I>) -> Result<Self, String> {
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

        // use crate::engine::graphics::{set_scaling_mode, ScalingMode};

        // set_scaling_mode(x, ScalingMode::Stretch, Some(crate::SCALE));

        let debug_font = gfx.create_font(include_bytes!("./Ubuntu-B.ttf"))?;

        let (sender, receiver) = event::split();

        let (console, processor) = Console::new();

        Ok(Self {
            state: MainStates::default(),

            loading: LoadingStateManager::new(gfx, sender.clone(), debug_font)?,

            title: TitleState::new(gfx, sender.clone())?,
            game: GameStateInit::Uninit(debug_font, sender.clone(), processor, 0),

            console,

            save: save::PlayerSave::None,
            // load
            //     .save
            //     .map(save::PlayerSave::Uninit)
            //     .unwrap_or(save::PlayerSave::None),
            data: load,

            receiver,
            sender,
            // scaler,
        })
    }

    pub fn start(&mut self, app: &mut App, plugins: &mut Plugins) {
        match self.state {
            MainStates::Loading => (), //self.loading.start()
            MainStates::Title => self.title.start(app, plugins),
            MainStates::Menu => (),
            MainStates::Game => {
                if let LoadData::Data {
                    pokedex,
                    movedex,
                    itemdex,
                } = &self.data
                {
                    if !self
                        .save
                        .init(&mut rand::thread_rng(), pokedex, movedex, itemdex)
                    {
                        worldcli::engine::log::info!("Could not init player; {:?}", self.save)
                    }
                } else {
                    worldcli::engine::log::info!("Could not init player, no dexes; {:?}", self.save)
                }

                if let Some(player) = self.save.as_mut() {
                    if let GameStateInit::Init(game) = &mut self.game {
                        game.start(player);
                    } else {
                        self.state = MainStates::Menu;
                    }
                } else {
                    self.state = MainStates::Menu;
                }
            }
        }
    }
    pub fn end(&mut self, app: &mut App, plugins: &mut Plugins) {
        match self.state {
            MainStates::Loading => (),
            MainStates::Title => self.title.end(app, plugins),
            MainStates::Menu => (),
            MainStates::Game => {
                if let GameStateInit::Init(game) = &mut self.game {
                    game.end(app, plugins)
                }
            }
        }
        self.process(app, plugins);
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins) {
        let delta = app.timer.delta_f32();

        match self.state {
            MainStates::Loading => {
                if pressed(app, plugins, Control::A) {
                    self.state = MainStates::Title;
                }

                self.loading.update(app, plugins, delta);
            }
            MainStates::Title => self.title.update(app, plugins),
            MainStates::Menu => (),
            MainStates::Game => {
                if let Some(player) = self.save.as_mut() {
                    if let LoadData::Data {
                        pokedex,
                        movedex,
                        itemdex,
                    } = &self.data
                    {
                        if let GameStateInit::Init(game) = &mut self.game {
                            game.update(app, plugins, pokedex, movedex, itemdex, player);
                        } else {
                            self.state = MainStates::Menu;
                        }
                    }
                } else {
                    self.state = MainStates::Menu;
                }
            }
        }

        self.process(app, plugins);

        // Ok(())
    }
    pub fn draw(&mut self, app: &mut App, plugins: &mut Plugins, gfx: &mut Graphics) {
        // graphics::set_canvas(ctx, self.scaler.canvas());
        // graphics::clear(ctx, Color::BLACK);

        match self.state {
            MainStates::Loading | MainStates::Title | MainStates::Menu => {
                if let GameStateInit::Uninit(..) = &self.game {
                    if let Some(data) = self.data.finish() {
                        let game = std::mem::replace(&mut self.game, GameStateInit::None);
                        if let GameStateInit::Uninit(debug_font, sender, processor, seed) = game {
                            self.game = GameStateInit::Init({
                                let btl = firecore_battle_engine::BattleGuiData::new(gfx).unwrap();
                                let mut game = GameStateManager::new(
                                    app,
                                    gfx,
                                    plugins,
                                    debug_font,
                                    data.dex,
                                    btl,
                                    data.world,
                                    data.battle,
                                    sender,
                                    processor,
                                )
                                .unwrap();
                                game.seed(seed as _);
                                game
                            });
                        }
                    }
                }
            }
            _ => (),
        }

        match self.state {
            MainStates::Loading => self.loading.draw(gfx),
            MainStates::Title => self.title.draw(gfx),
            MainStates::Menu => {
                let mut draw = gfx.create_draw();
                draw.clear(Color::new(0.00, 0.32, 0.67, 1.0));
                gfx.render(&draw);
            }
            MainStates::Game => {
                if let Some(player) = self.save.as_mut() {
                    if let GameStateInit::Init(game) = &self.game {
                        game.draw(gfx, player);
                    }
                }
            }
        }

        match &self.state {
            MainStates::Loading | MainStates::Title | MainStates::Menu => {
                if let LoadData::Load(assets, ..) = &self.data {
                    let mut draw = gfx.create_draw();

                    draw.rect(
                        (draw.width() / 5.0, draw.height() - draw.height() / 5.0),
                        (
                            draw.width() * 3.0 / 5.0 * assets.progress(),
                            draw.height() / 15.0,
                        ),
                    )
                    .color(Color::WHITE);

                    gfx.render(&draw);
                }
            }
            _ => (),
        }

        use crate::engine::notan::egui::{self, EguiPluginSugar};
        let plugins2 = unsafe { &mut *(plugins as *mut _) };
        gfx.render(&plugins.egui(|egui| {
            match self.state {
                MainStates::Menu => {
                    egui::Window::new("Menu").show(egui, |ui| {
                        if ui.button("Play").clicked() {
                            self.sender.send(StateMessage::Goto(MainStates::Game));
                        }
                        if ui.button("New Save / Reset Save").clicked() {
                            self.sender.send(StateMessage::ResetSave);
                        }
                        if ui.button("Exit").clicked() {
                            self.sender.send(StateMessage::Exit);
                        }
                    });
                }
                MainStates::Game => {
                    if let Some(player) = self.save.as_mut() {
                        if let GameStateInit::Init(game) = &mut self.game {
                            game.ui(app, plugins2, egui, player);
                        }
                    }
                }
                _ => (),
            }

            Touchscreen::ui(egui, plugins2);

            self.console.ui::<P, M, I>(app, egui, self.save.as_mut());
        }));

        // graphics::reset_transform_matrix(ctx);
        // graphics::reset_canvas(ctx);
        // graphics::clear(ctx, Color::BLACK);
        // self.scaler.draw(ctx);
        // Ok(())
    }

    pub fn process(&mut self, app: &mut App, plugins: &mut Plugins) {
        while let Some(message) = self.receiver.read() {
            match message {
                StateMessage::ResetSave => {
                    self.save = save::PlayerSave::Uninit(Player::new("Red", "Blue"));
                }
                StateMessage::SaveToDisk => {
                    // if let Some(save) = self.save.cloned_uninit() {
                    //     if let Err(err) = storage::save::<storage::RonSerializer, Player>(
                    //         &save,
                    //         crate::PUBLISHER,
                    //         crate::APPLICATION,
                    //     ) {
                    //         error!("Cannot write save with error {}", err);
                    //     }
                    // }
                }
                StateMessage::Seed(seed) => match &mut self.game {
                    GameStateInit::Uninit(.., s) => *s = seed,
                    GameStateInit::Init(game) => game.seed(seed as _),
                    GameStateInit::None => (),
                },
                StateMessage::Goto(state) => {
                    self.end(app, plugins);
                    self.state = state;
                    self.start(app, plugins);
                }
                StateMessage::Exit => app.exit(),
            }
        }
    }
}
