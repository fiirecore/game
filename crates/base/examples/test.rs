use engine::{
    gui::MessageBox,
    text::{MessagePage, MessageState, MessageStates},
};
use firecore_base as engine;
use notan::prelude::*;
use notan::{draw::DrawConfig, egui::*};

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(Game::new)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .update(Game::update)
        .draw(Game::draw)
        .build()
    // engine::run(
    //     ContextBuilder::new(
    //         "MessageBox",
    //         (SCALE * engine::utils::WIDTH) as i32,
    //         (SCALE * engine::utils::HEIGHT) as _,
    //     ),
    //     async {},
    //     |ctx, eng, ()| {
    //         // let fonts: Vec<FontSheet<Vec<u8>>> =
    //         //     firecore_storage::from_bytes(include_bytes!("fonts.bin")).unwrap();

    //         // let mut audio: engine::context::audio::SerializedAudio =
    //         //     firecore_storage::from_bytes(include_bytes!("audio.bin")).unwrap();

    //         let id = "battle_wild".parse().unwrap();

    //         engine::music::add_music(ctx, eng, id, vec![]);

    //         // engine::context::audio::GameAudio::init(ctx, audio).await;

    //         // engine::music::play_music(ctx, &id);

    //         // for font_sheet in fonts {
    //         //     engine::text::insert_font(ctx, eng, &font_sheet).unwrap();
    //         // }
    //     },
    //     |_, _, _| Game::new(),
    // )
}

#[derive(AppState)]
struct Game {
    state: MessageStates<Color>,
}

impl Game {
    pub fn new(plugins: &mut Plugins) -> Self {
        engine::setup(plugins);
        Self {
            state: {
                let page = MessagePage {
                    lines: vec![
                        "Test Pagé Test Page".to_owned(),
                        "Pagé Test Page Test".to_owned(),
                    ],
                    wait: None,
                    color: Some(Color::RED),
                };
                let page2 = MessagePage {
                    lines: vec![
                        "Test Pagé Test Page 2".to_owned(),
                        "Pagé Test Page Test 2".to_owned(),
                    ],
                    wait: Some(1.0),
                    color: Some(Color::YELLOW),
                };

                MessageStates::Running(MessageState {
                    pages: vec![page, page2],
                    ..Default::default()
                })
            },
        }
    }
}

impl Game {
    fn update(app: &mut App, game: &mut Game) {
        //-> Result {
        if !game.state.is_running() {
            app.exit();
            // game.messagebox
            //     .update(app, plugins, app.timer.delta_f32(), &mut game.state);
        }
        // Ok(())
    }

    fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, game: &mut Game) {
        //-> Result<(), ()> {

        // graphics::clear(ctx, Color::rgb(0.1, 0.2, 0.56));
        // Panel::draw(
        //     ctx,
        //     eng,
        //     10.0,
        //     10.0,
        //     engine::utils::WIDTH - 20.0,
        //     engine::utils::HEIGHT - 20.0,
        // );
        let plugins2 = unsafe { &mut *(plugins as *mut _) };
        gfx.render(&plugins.egui(|egui| MessageBox::ui(app, plugins2, egui, &mut game.state)));
        // Ok(())
    }
}
