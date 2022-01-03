use engine::{
    graphics::{self, Color, ScalingMode},
    gui::{MessageBox, Panel},
    text::MessagePage,
    utils::{Completable, Entity},
    Context, ContextBuilder, State, EngineContext,
};
use firecore_engine as engine;

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
            let fonts: Vec<_> = bincode::deserialize(include_bytes!("fonts.bin")).unwrap();

            // let mut audio: engine::context::audio::SerializedAudio =
            //     bincode::deserialize(include_bytes!("audio.bin")).unwrap();

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
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default(), 0),
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
            ],
            wait: None,
            color: Color::RED,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
            color: Color::GOLD,
        };
        // self.messagebox.pages.push(page);
        self.messagebox.pages.extend([page, page2]);
        self.messagebox.spawn();
        // Ok(())
    }

    fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        //-> Result {
        if !self.messagebox.alive() {
            ctx.quit();
        } else {
            self.messagebox.update(ctx, eng, delta);
            if self.messagebox.finished() {
                self.messagebox.despawn();
            }
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
        self.messagebox.draw(ctx, eng);
        // Ok(())
    }
}
