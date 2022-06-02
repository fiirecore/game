use firecore_battle_engine::pokengine::engine::{
    self,
    graphics::{self, Color, Graphics},
    gui::MessageBox,
    text::{MessagePage, MessageState},
};
use firecore_pokedex_engine::engine::Plugins;

const SCALE: f32 = 2.0;

fn main() {}

struct Game {
    messagebox: MessageBox,
    state: Option<MessageState<Color>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default()),
            state: None,
        }
    }
}

impl Game {
    fn start(&mut self, ctx: &mut Context, _: &mut EngineContext) {
        engine::graphics::set_scaling_mode(ctx, ScalingMode::Stretch, Some(SCALE));

        //-> Result {
        let page = MessagePage {
            lines: vec![
                "Test Pagé Test Page".to_owned(),
                "Pagé Test Page Test".to_owned(),
            ],
            wait: None,
            color: Some(Color::RED),
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
            color: Some(Color::GOLD),
        };

        self.state = Some(MessageState::new(vec![page, page2]));

        // Ok(())
    }

    fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        //-> Result {
        if self.state.is_some() {
            self.messagebox.update(ctx, eng, delta, &mut self.state);
        }
        // Ok(())
    }

    pub fn draw(&mut self, plugins: &mut Plugins, gfx: &mut Graphics) {
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
