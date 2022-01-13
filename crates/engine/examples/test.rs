use engine::{
    graphics::{self, Color, ScalingMode},
    gui::{MessageBox, Panel},
    text::{MessagePage, MessageState},
    Context, ContextBuilder, EngineContext, State,
};
use firecore_engine as engine;
use firecore_font_builder::{FontId, FontSheet};

const SCALE: f32 = 2.0;

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            (SCALE * engine::utils::WIDTH) as i32,
            (SCALE * engine::utils::HEIGHT) as _,
        ),
        async {},
        |ctx, eng, ()| {
            let fonts: Vec<FontSheet<Vec<u8>>> = firecore_storage::from_bytes(include_bytes!("fonts.bin")).unwrap();

            // let mut audio: engine::context::audio::SerializedAudio =
            //     firecore_storage::from_bytes(include_bytes!("audio.bin")).unwrap();

            let id = "battle_wild".parse().unwrap();

            engine::music::add_music(ctx, eng, id, vec![]);

            // engine::context::audio::GameAudio::init(ctx, audio).await;

            // engine::music::play_music(ctx, &id);

            for font_sheet in fonts {
                engine::text::insert_font(ctx, eng, &font_sheet).unwrap();
            }
        },
        |_, _, _| Game::new(),
    )
}

struct Game {
    messagebox: MessageBox,
    state: Option<MessageState<FontId, Color>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default()),
            state: None,
        }
    }
}

impl State<EngineContext> for Game {
    fn start(&mut self, ctx: &mut Context, _: &mut EngineContext) {
        engine::graphics::set_scaling_mode(ctx, ScalingMode::Stretch, Some(SCALE));

        //-> Result {
        let page = MessagePage {
            lines: vec![
                "Test Pagé Test Page".to_owned(),
                "Pagé Test Page Test".to_owned(),
                "Test Pagé Test Page 2".to_owned(),
                "Pagé Test Page Test 2".to_owned(),
            ],
            wait: None,
            color: Color::RED,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
            color: Color::GOLD,
        };

        self.state = Some(MessageState::from(vec![page, page2]));

        // Ok(())
    }

    fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        //-> Result {
        if self.state.is_some() {
            self.messagebox.update(ctx, eng, delta, &mut self.state);
        }
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        //-> Result<(), ()> {
        graphics::clear(ctx, Color::rgb(0.1, 0.2, 0.56));
        Panel::draw(
            ctx,
            eng,
            10.0,
            10.0,
            engine::utils::WIDTH - 20.0,
            engine::utils::HEIGHT - 20.0,
        );
        self.messagebox.draw(ctx, eng, self.state.as_ref());
        // Ok(())
    }
}
