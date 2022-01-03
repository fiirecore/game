use firecore_battle_engine::pokedex::engine::{
    self,
    graphics::{self, Color, ScalingMode},
    gui::{MessageBox, Panel},
    text::MessagePage,
    utils::{Completable, Entity},
    Context, ContextBuilder, State,
};
use firecore_pokedex_engine::engine::EngineContext;

const SCALE: f32 = 2.0;

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            (SCALE * engine::utils::WIDTH) as _,
            (SCALE * engine::utils::HEIGHT) as _,
        ),
        async {},
        move |_, _, _| {},
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
        graphics::set_scaling_mode(ctx, ScalingMode::Stretch, Some(SCALE));

        //-> Result {
        let page = MessagePage {
            lines: vec![
                "Test Page Test Page".to_owned(),
                "Page Test Page Test".to_owned(),
            ],
            wait: None,
            color: MessagePage::BLACK,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
            color: MessagePage::BLACK,
        };
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
