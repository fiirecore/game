use firecore_battle_engine::pokedex::engine::{
    self,
    graphics::{self, Color, ScalingMode},
    gui::{MessageBox, Panel},
    text::MessagePage,
    Context, ContextBuilder, State,
};
use firecore_pokedex_engine::engine::{EngineContext, text::{MessageState, FontId}};

const SCALE: f32 = 2.0;

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            (SCALE * 240.0) as _,
            (SCALE * 160.0) as _,
        ),
        async {},
        move |_, _, _| {},
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
            ],
            wait: None,
            color: Color::RED,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
            color: Color::GOLD,
        };

        self.state = Some(MessageState::new(1, vec![page, page2]));

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
